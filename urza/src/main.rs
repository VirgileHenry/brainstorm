mod model;
mod node_id;

type BurnBackend = burn::backend::Wgpu<f32, i32>;

fn main() {
    let start = chrono::Utc::now();
    println!("Sarting training at {}", start);

    let now = chrono::Utc::now();
    let cards = load_cards();
    println!(
        "Loaded {} cards in {}sec",
        cards.len(),
        (chrono::Utc::now() - now).as_seconds_f32()
    );

    let now = chrono::Utc::now();
    let model = load_model();
    println!(
        "Loaded neural network model in {}sec:\n{model}",
        (chrono::Utc::now() - now).as_seconds_f32()
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

fn load_model() -> model::Card2Vec<BurnBackend> {
    let device = Default::default();
    let config = model::Card2VecConfig::new();
    config.init::<BurnBackend>(&device)
}
