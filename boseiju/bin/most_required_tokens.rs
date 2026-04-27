//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.
use boseiju::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    const SHOWN_TOKENS: usize = 20;
    let cards = mtg_cardbase::AllCardsIter::hexxed_v1_cards();
    let cards_vec: Vec<_> = cards.iter().collect();

    let num_threads = rayon::current_num_threads();
    let chunk_size = cards_vec.len().div_ceil(num_threads);

    let multi = indicatif::MultiProgress::new();
    let template = "Thread {prefix:>2} [{bar:40.yellow/white}] {pos:>6}/{len:6} ({percent}%)";
    let style = indicatif::ProgressStyle::with_template(template)
        .unwrap()
        .progress_chars("─● ");

    let chunks: Vec<_> = cards_vec.chunks(chunk_size).collect();
    let bars: Vec<indicatif::ProgressBar> = chunks
        .iter()
        .enumerate()
        .map(|(i, chunk)| {
            let pb = multi.add(indicatif::ProgressBar::new(chunk.len() as u64));
            pb.set_style(style.clone());
            pb.set_prefix(format!("{i}"));
            pb
        })
        .collect();

    let most_required_tokens: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());

    chunks.par_iter().zip(bars.par_iter()).for_each(|(chunk, pb)| {
        let mut local: HashMap<String, usize> = HashMap::new();
        let total = chunk.len();
        let mut last_shown_percentage = 0usize;

        for (i, card) in chunk.iter().enumerate() {
            let progress = (i + 1) * 100 / total;
            if progress != last_shown_percentage {
                pb.set_position(((i + 1) as u64).min(total as u64));
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

            *local.entry(token).or_default() += 1;
        }

        let done_template = "Thread {prefix:>2} [{bar:40.green/white}] {pos:>6}/{len:6} ({percent}%)";
        let style = indicatif::ProgressStyle::with_template(done_template)
            .unwrap()
            .progress_chars("─● ");
        pb.set_style(style);
        pb.finish_with_message("done");

        // Single lock per thread: drain local counts into the shared map.
        let mut shared = most_required_tokens.lock().unwrap();
        for (key, count) in local {
            *shared.entry(key).or_default() += count;
        }
    });

    let most_required_tokens = most_required_tokens.into_inner().unwrap();
    let mut most_required_tokens: Vec<_> = most_required_tokens.into_iter().collect();
    most_required_tokens.sort_by(|(_, a), (_, b)| b.cmp(a));

    println!("");
    println!(
        "Most required tokens: (showing {SHOWN_TOKENS} out of {})",
        most_required_tokens.len()
    );
    for (token, count) in most_required_tokens.iter().take(SHOWN_TOKENS) {
        println!("{token} ({count} cards)");
    }
}
