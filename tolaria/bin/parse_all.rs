fn main() {
    // Ideally, parse all and write the results to file
    // For now, let's just keep track of how many cards we can parse!

    let mut parsed = Vec::new();
    let mut failed = Vec::new();

    let cards = mtg_cardbase::AllCardsIter::new();

    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    let results: Vec<_> = cards
        .par_iter()
        .filter(|card| filter_cards(card))
        .map(|card| tolaria::Card::try_from(card))
        .collect();

    for result in results {
        match result {
            Ok(card) => parsed.push(card),
            Err(err) => failed.push(err),
        }
    }

    println!(
        "Parsed {}/{} cards!",
        parsed.len(),
        parsed.len() + failed.len()
    );

    if parsed.len() > 0 {
        println!("First few successes are:");
        for card in parsed.iter().take(5) {
            println!("{card}");
        }
    }

    if failed.len() > 0 {
        println!("First few errors are:");
        for err in failed.iter().take(20) {
            println!("{err}");
        }
    }
}

fn filter_cards(card: &mtg_cardbase::Card) -> bool {
    const LAYOUTS: &[&'static str] = &[
        "normal",
        "split",
        "flip",
        "transform",
        "modalDfc",
        "meld",
        "leveler",
        "class",
        "case",
        "saga",
        "adventure",
        "mutate",
        "prototype",
        "battle",
        "planar",
        "scheme",
        "vanguard",
        "token",
        "doublefaced",
        "emblem",
    ];

    if !LAYOUTS.contains(&card.layout) {
        return false;
    }

    true
}
