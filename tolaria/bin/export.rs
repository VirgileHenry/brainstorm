/// Export all MTG cards into parsed json files.
#[derive(argh::FromArgs)]
struct ExportArgs {
    /// the output folder to write the json files to.
    #[argh(option, short = 'o')]
    output: String,
}

fn main() {
    let args: ExportArgs = argh::from_env();

    let cards = mtg_cardbase::AllCardsIter::new();

    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    cards.par_iter().filter(|card| filter_cards(card)).for_each(
        |card| match tolaria::Card::try_from(card) {
            Ok(parsed) => {
                let output_path = format!("{}/{}.json", args.output, card.name);
                match serde_json::to_string_pretty(&parsed) {
                    Ok(card_json) => match std::fs::write(&output_path, &card_json) {
                        Ok(_) => {}
                        Err(err) => eprintln!("Failed to write file:  {err}"),
                    },
                    Err(err) => eprintln!("Failed to convert card to json: {err}"),
                }
            }
            Err(err) => eprintln!("Failed to parse card {}: {err}", card.name),
        },
    );
}

fn filter_cards(card: &mtg_cardbase::Card) -> bool {
    card.set == "fdn"
}
