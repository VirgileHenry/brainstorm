mod flatten;

fn main() {
    let start = chrono::Utc::now();
    println!("Sarting training at {}", start);

    let cards = load_cards();
    println!(
        "Loaded {} cards in {}sec",
        cards.len(),
        (chrono::Utc::now() - start).as_seconds_f32()
    );
}

fn load_cards() -> Vec<tolaria::Card> {
    /* For now, we can parse and load up the cards dynamically, as it takes less than a second to do so. */
    let cards = mtg_cardbase::AllCardsIter::new();

    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;

    cards
        .par_iter()
        .filter(|card| card.set == "fdn")
        .map(|card| tolaria::Card::try_from(card))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}
