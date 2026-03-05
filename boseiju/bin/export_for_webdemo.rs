//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.

use boseiju::*;

struct ExportableCard {
    card_name: String,
    oracle_text: String,
    score: usize,
}

fn main() {
    let cards = mtg_cardbase::AllCardsIter::new();

    const EXPORT_CARD_COUNT: usize = 100;
    let mut result = Vec::new();

    let mut last_shown_percentage = 0;
    eprintln!("Parsing all cards...0%");
    for (i, card) in cards.iter().enumerate() {
        let progress = (i + 1) * 100 / cards.len();
        if progress != last_shown_percentage {
            eprintln!("\rParsing all cards...{progress}%");
            last_shown_percentage = progress;
        }

        let card_name = card.name.clone();
        let oracle_text = match card.oracle_text.as_ref() {
            Some(text) => text,
            None => continue,
        };
        let oracle_text = lexer::preprocess(card_name.as_str(), oracle_text);
        let tokens = match lexer::lex(&oracle_text) {
            Ok(tokens) => tokens,
            Err(_) => continue,
        };
        let _ = match parser::parse(&tokens) {
            Ok(tree) => tree,
            Err(_) => continue,
        };

        let mut to_export = ExportableCard {
            card_name,
            oracle_text: oracle_text.replace('\n', "\\n"),
            score: tokens.len(),
        };
        match result.get_mut(EXPORT_CARD_COUNT) {
            None => result.push(to_export),
            Some(prev_last) => {
                if to_export.score > prev_last.score {
                    std::mem::swap(prev_last, &mut to_export);
                }
            }
        }

        result.sort_by(|a, b| b.score.cmp(&a.score));
    }

    println!("const EXAMPLE_CARDS = [");
    for card in result.into_iter() {
        println!("  {{");
        println!("    name: \"{}\",", card.card_name);
        println!("    oracle: \"{}\",", card.oracle_text);
        println!("  }},");
    }
    println!("];");

    eprintln!("done !");
}
