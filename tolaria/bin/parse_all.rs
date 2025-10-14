fn main() {
    // Ideally, parse all and write the results to file
    // For now, let's just keep track of how many cards we can parse!

    let mut parsed = Vec::new();
    let mut failed = Vec::new();

    for card in mtg_cardbase::AllCardsIter::new() {
        match tolaria::Card::try_from(&card) {
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
            println!("{card:#?}");
        }
    }

    if failed.len() > 0 {
        println!("First few errors are:");
        for err in failed.iter().take(5) {
            println!("{err}");
        }
    }
}
