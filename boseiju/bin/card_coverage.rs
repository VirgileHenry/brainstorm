//! Perform the card coverage test, and outputs a markdown table with a recap.

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
            },
            TestResult::FullCardParsed => {
                self.oracle_text_lexed += 1;
                self.oracle_text_parsed += 1;
                self.fully_parsed += 1;
            },
        }
    }
}

fn main() -> std::io::Result<()> {
    /* Run the test coverage once, on all the cards */
    let cards = mtg_cardbase::AllCardsIter::new();
    let mut cards_parsing_results = Vec::with_capacity(cards.len());

    let mut last_shown_percentage = 0;
    eprintln!("Parsing all cards...0%");
    for (i, card) in cards.iter().enumerate() {
        let progress = (i + 1) * 100 / cards.len();
        if progress != last_shown_percentage {
            eprintln!("\rParsing all cards...{progress}%");
            last_shown_percentage = progress;
        }
        cards_parsing_results.push((card, run_card(card)))
    }

    /* Create the different coverage categories */
    let categories = vec![
        CoverageTestCase::new("Foundation set (FDN)", Box::new(|card| card.set == "fdn")),
        CoverageTestCase::new("Last set (TLA)", Box::new(|card| card.set == "tla")), // Fixme: fetch last set ?
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
    println!("| Category | Cards total | Lexed (oracle text) | Parsed (oracle text) | Parsed (full card) |");
    println!("|-----|-----|-----|-----|-----|");

    for (category, results) in categories_results.iter() {
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

fn card_legal_in(card: &mtg_cardbase::Card, format: mtg_data::Format) -> bool {
    let legality = match format {
        mtg_data::Format::Commander => &card.legalities.commander,
        mtg_data::Format::Standard => &card.legalities.standard,
        _ => unreachable!(),
    };
    legality == "legal" || legality == "restricted"
}
