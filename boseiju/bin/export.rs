//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.

use std::io::Write;

use boseiju::*;

fn main() -> std::io::Result<()> {
    let cards = mtg_cardbase::AllCardsIter::new();

    const MAX_ERRORS_SHOWN: usize = 10;
    const OUTPUT_FILE: &'static str = "boseiju_export.json";

    let mut result = Vec::new();
    let mut errors_shown = 0;

    let mut last_shown_percentage = 0;
    eprintln!("Parsing all cards...0%");
    for (i, card) in cards.iter().enumerate() {
        let progress = (i + 1) * 100 / cards.len();
        if progress != last_shown_percentage {
            eprintln!("\rParsing all cards...{progress}%");
            last_shown_percentage = progress;
        }

        let parsed_card = Card::try_from(card);

        match parsed_card {
            Ok(card) => result.push(card),
            Err(e) => {
                if errors_shown < MAX_ERRORS_SHOWN {
                    eprintln!("Failed to parse: {e}");
                    errors_shown += 1;
                }
            }
        }
    }

    eprintln!("Writing {} cards to {}", result.len(), OUTPUT_FILE);

    let serialized = serde_json::to_string(&result)?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(OUTPUT_FILE)?;
    file.write_all(serialized.as_bytes())?;

    eprintln!("Done !");

    Ok(())
}
