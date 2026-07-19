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
    JsonParsed,
    OracleTextLexed,
    FullCardParsed,
}

#[derive(Default)]
struct CoverageTestResults {
    total: usize,
    json_parsed: usize,
    oracle_text_lexed: usize,
    fully_parsed: usize,
}

impl CoverageTestResults {
    fn add_result(&mut self, result: &TestResult) {
        match result {
            TestResult::Failed => {
                self.total += 1;
            }
            TestResult::JsonParsed => {
                self.total += 1;
                self.json_parsed += 1;
            }
            TestResult::OracleTextLexed => {
                self.total += 1;
                self.json_parsed += 1;
                self.oracle_text_lexed += 1
            }
            TestResult::FullCardParsed => {
                self.total += 1;
                self.json_parsed += 1;
                self.oracle_text_lexed += 1;
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
        CoverageTestCase::new("Last set (HOB)", Box::new(|card| card.set == "hob")), // Fixme: fetch last set ?
        CoverageTestCase::new(
            "Standard-legal cards",
            Box::new(|card| card_legal_in(card, mtg_data::Format::Standard)),
        ),
        CoverageTestCase::new(
            "Commander-legal cards",
            Box::new(|card| card_legal_in(card, mtg_data::Format::Commander)),
        ),
        CoverageTestCase::new("All paper cards", Box::new(|_| true)),
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
    println!("| Category | Cards total | JSON Parsed | Lexed | Parsed |");
    println!("|-----|-----|-----|-----|-----|");

    for (category, results) in categories_results.iter() {
        if results.total > 0 {
            println!(
                "|{}|{}|{} ({}%)|{} ({}%)|{} ({}%)|",
                category.name,
                results.total,
                results.json_parsed,
                results.json_parsed * 100 / results.total,
                results.oracle_text_lexed,
                results.oracle_text_lexed * 100 / results.json_parsed,
                results.fully_parsed,
                results.fully_parsed * 100 / results.oracle_text_lexed,
            );
        } else {
            println!("|{}|0|skipped|skipped|skipped|", category.name);
        }
    }

    Ok(())
}

fn run_card(card: &mtg_cardbase::Card) -> TestResult {
    use boseiju_parser::CardParseError;
    use boseiju_parser::LayoutParseError;

    match boseiju_parser::parse_card(card) {
        Err(CardParseError::InvalidJson(_)) => TestResult::Failed,
        Err(CardParseError::InvalidLayout(LayoutParseError::InvalidColors(_))) => TestResult::Failed,
        Err(CardParseError::InvalidLayout(LayoutParseError::InvalidManaCost { .. })) => TestResult::Failed,
        Err(CardParseError::InvalidLayout(LayoutParseError::InvalidTypeLine { .. })) => TestResult::Failed,
        Err(CardParseError::InvalidLayout(LayoutParseError::UnknownLayout { .. })) => TestResult::Failed,
        Err(CardParseError::InvalidLayout(LayoutParseError::LexerError(_))) => TestResult::JsonParsed,
        Err(CardParseError::InvalidLayout(LayoutParseError::ParserError(_))) => TestResult::OracleTextLexed,
        Ok(_) => TestResult::FullCardParsed,
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
