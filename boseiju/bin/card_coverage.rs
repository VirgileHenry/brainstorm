//! Perform the card coverage test, and outputs a markdown table with a recap.

use rayon::prelude::*;

struct CoverageTestCase {
    name: &'static str,
    filter_func: Box<dyn Fn(&mtg_cardbase::Card) -> bool>,
}

impl CoverageTestCase {
    fn new(name: &'static str, filter_func: Box<dyn Fn(&mtg_cardbase::Card) -> bool>) -> Self {
        Self { name, filter_func }
    }
}

enum TestResult {
    Failed,
    OracleTextLexed,
    OracleTextParsed,
    FullCardParsed,
}

#[derive(Default)]
struct CoverageTestResults {
    total: usize,
    oracle_text_lexed: usize,
    oracle_text_parsed: usize,
    fully_parsed: usize,
}

impl CoverageTestResults {
    fn add_result(&mut self, result: &TestResult) {
        self.total += 1;
        match result {
            TestResult::Failed => { /* :( */ }
            TestResult::OracleTextLexed => self.oracle_text_lexed += 1,
            TestResult::OracleTextParsed => {
                self.oracle_text_lexed += 1;
                self.oracle_text_parsed += 1;
            }
            TestResult::FullCardParsed => {
                self.oracle_text_lexed += 1;
                self.oracle_text_parsed += 1;
                self.fully_parsed += 1;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    /* Run the test coverage once, on all the cards */
    let cards = mtg_cardbase::AllCardsIter::hexxed_v1_cards();
    let cards_vec: Vec<_> = cards.iter().collect();

    let num_threads = rayon::current_num_threads();
    let chunk_size = cards_vec.len().div_ceil(num_threads);

    let multi = indicatif::MultiProgress::new();
    let template = "Thread {prefix:>2} [{bar:40.yellow/white}] {pos:>6}/{len:6} ({percent}%)";
    let style = indicatif::ProgressStyle::with_template(template)
        .unwrap()
        .progress_chars("─● ");

    let chunks: Vec<_> = cards_vec.chunks(chunk_size).collect();
    let bars: Vec<indicatif::ProgressBar> = chunks
        .iter()
        .enumerate()
        .map(|(i, chunk)| {
            let pb = multi.add(indicatif::ProgressBar::new(chunk.len() as u64));
            pb.set_style(style.clone());
            pb.set_prefix(format!("{i}"));
            pb
        })
        .collect();

    // Each thread produces its own Vec; we concatenate at the end.
    // No shared mutex needed — the result vec just preserves per-card order within a chunk.
    let cards_parsing_results: Vec<(&mtg_cardbase::Card, TestResult)> = chunks
        .par_iter()
        .zip(bars.par_iter())
        .flat_map(|(chunk, pb)| {
            let total = chunk.len();
            let mut last_shown_percentage = 0usize;
            let mut local: Vec<(&mtg_cardbase::Card, TestResult)> = Vec::with_capacity(total);

            for (i, card) in chunk.iter().enumerate() {
                let progress = (i + 1) * 100 / total;
                if progress != last_shown_percentage {
                    pb.set_position(((i + 1) as u64).min(total as u64));
                    last_shown_percentage = progress;
                }

                local.push((*card, run_card(card)));
            }

            let done_template = "Thread {prefix:>2} [{bar:40.green/white}] {pos:>6}/{len:6} ({percent}%)";
            let style = indicatif::ProgressStyle::with_template(done_template)
                .unwrap()
                .progress_chars("─● ");
            pb.set_style(style);
            pb.finish_with_message("done");
            local
        })
        .collect();

    /* Create the different coverage categories */
    let categories = vec![
        CoverageTestCase::new("Foundation set (FDN)", Box::new(|card| card.set == "fdn")),
        CoverageTestCase::new("Last set (ECL)", Box::new(|card| card.set == "ecl")), // Fixme: fetch last set ?
        CoverageTestCase::new(
            "Standard-legal cards",
            Box::new(|card| card_legal_in(card, mtg_data::Format::Standard)),
        ),
        CoverageTestCase::new(
            "Commander-legal cards",
            Box::new(|card| card_legal_in(card, mtg_data::Format::Commander)),
        ),
        CoverageTestCase::new("All (except uncards)", Box::new(|card| card.set_type != "funny")),
    ];

    let categories_results = categories
        .iter()
        .map(|category| {
            let mut results = CoverageTestResults::default();
            for (card, result) in cards_parsing_results.iter() {
                if (category.filter_func)(card) {
                    results.add_result(result);
                }
            }
            (category, results)
        })
        .collect::<Vec<_>>();

    /* Finally, we can display the output */
    println!("");
    println!("| Category | Cards total | Lexed (oracle text) | Parsed (oracle text) | Parsed (full card) |");
    println!("|-----|-----|-----|-----|-----|");

    for (category, results) in categories_results.iter() {
        if results.total > 0 {
            println!(
                "|{}|{}|{} ({}%)|{} ({}%)|{} ({}%)|",
                category.name,
                results.total,
                results.oracle_text_lexed,
                results.oracle_text_lexed * 100 / results.total,
                results.oracle_text_parsed,
                results.oracle_text_parsed * 100 / results.total,
                results.fully_parsed,
                results.fully_parsed * 100 / results.total,
            );
        } else {
            println!("|{}|0|skipped|skipped|skipped|", category.name);
        }
    }

    Ok(())
}

fn run_card(card: &mtg_cardbase::Card) -> TestResult {
    let card_name = &card.name;
    let oracle_text = match card.oracle_text.as_ref() {
        Some(ot) => ot,
        /* If there is no oracle text, attempt to raw parse */
        None => match boseiju::Card::try_from(card) {
            Ok(_) => return TestResult::FullCardParsed,
            Err(_) => return TestResult::Failed,
        },
    };

    let preprocessed = boseiju::preprocess(card_name, oracle_text);
    let tokens = match boseiju::lex(&preprocessed) {
        Ok(lexed) => lexed,
        Err(_) => return TestResult::Failed,
    };

    match boseiju::parse(&tokens) {
        Ok(parsed) => parsed,
        Err(_) => return TestResult::OracleTextLexed,
    };

    match boseiju::Card::try_from(card) {
        Ok(_) => return TestResult::FullCardParsed,
        Err(_) => return TestResult::OracleTextParsed,
    }
}

/// Doc
fn card_legal_in(card: &mtg_cardbase::Card, format: mtg_data::Format) -> bool {
    let legality = match format {
        mtg_data::Format::Commander => &card.legalities.commander,
        mtg_data::Format::Standard => &card.legalities.standard,
        _ => unreachable!(),
    };
    legality == "legal" || legality == "restricted"
}
