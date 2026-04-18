//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.

use boseiju::*;

#[derive(Default)]
struct MostRequiredNode {
    count: usize,
    as_in: Vec<String>,
}

fn main() {
    const SHOWN_NODES: usize = 20;

    let cards = mtg_cardbase::AllCardsIter::hexxed_v1_cards();
    let mut most_required_nodes = std::collections::HashMap::<&str, MostRequiredNode>::new();

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
        let preprocessed = lexer::preprocess(card_name.as_str(), oracle_text);
        let tokens = match lex(&preprocessed) {
            Ok(preprocessed) => preprocessed,
            Err(_) => continue,
        };

        let token = match parse(&tokens) {
            Err(parser::ParserError::UnexpectedToken { found, .. }) => found,
            _ => continue,
        };

        let node = most_required_nodes.entry(token.name).or_default();
        node.count += 1;
        node.as_in.push(oracle_text.clone());
    }

    let mut most_required_tokens: Vec<_> = most_required_nodes.into_iter().collect();
    most_required_tokens.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

    const UNDERLINE_ON: &str = "\x1b[4m";
    const UNDERLINE_RESET: &str = "\x1b[24m";
    println!(
        "Most required nodes: (showing {SHOWN_NODES} out of {})",
        most_required_tokens.len()
    );
    for (token, node) in most_required_tokens.iter().take(SHOWN_NODES) {
        println!("{UNDERLINE_ON}{token}{UNDERLINE_RESET} ({} cards) as in:", node.count);
        for as_in in node.as_in.iter().take(5) {
            println!(" - {as_in:?}");
        }
        println!();
    }
}
