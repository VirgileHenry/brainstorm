//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.

use boseiju::*;

fn main() {
    const SHOWN_TOKENS: usize = 20;

    let cards = mtg_cardbase::AllCardsIter::new();
    let mut most_required_tokens = std::collections::HashMap::<String, usize>::new();

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
        let lexer_error = match lexer::lex(&oracle_text) {
            Ok(_) => continue,
            Err(err) => err,
        };

        let token = match lexer_error {
            lexer::LexerError::NoTokenMatch { tokens, .. } => match tokens.split(' ').next() {
                Some(first) => first.to_string(),
                None => tokens.to_string(),
            },
        };

        let count = most_required_tokens.entry(token).or_default();
        *count += 1;
    }

    let mut most_required_tokens: Vec<_> = most_required_tokens.into_iter().collect();
    most_required_tokens.sort_by(|(_, a), (_, b)| b.cmp(a));

    println!(
        "Most required tokens: (showing {SHOWN_TOKENS} out of {})",
        most_required_tokens.len()
    );
    for (token, count) in most_required_tokens.iter().take(SHOWN_TOKENS) {
        println!("{token} ({count} cards)");
    }
}
