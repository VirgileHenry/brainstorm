//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.

use boseiju::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
struct MostRequiredNode {
    count: usize,
    as_in: Vec<String>,
}

fn main() {
    const SHOWN_NODES: usize = 20;
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

    let most_required_nodes: Mutex<HashMap<&str, MostRequiredNode>> = Mutex::new(HashMap::new());

    chunks.par_iter().zip(bars.par_iter()).for_each(|(chunk, pb)| {
        let mut local: HashMap<&str, MostRequiredNode> = HashMap::new();
        let total = chunk.len();
        let mut last_shown_percentage = 0usize;

        for (i, card) in chunk.iter().enumerate() {
            // Only tick the bar when the integer percentage advances.
            let progress = (i + 1) * 100 / total;
            if progress != last_shown_percentage {
                pb.set_position(((i + 1) as u64).min(total as u64));
                last_shown_percentage = progress;
            }

            let oracle_text = match card.oracle_text.as_ref() {
                Some(text) => text,
                None => continue,
            };
            let preprocessed = lexer::preprocess(card.name.as_str(), oracle_text);
            let tokens = match lex(&preprocessed) {
                Ok(preprocessed) => preprocessed,
                Err(_) => continue,
            };
            let token = match parse(&tokens) {
                Err(parser::ParserError::UnexpectedToken { found, .. }) => found,
                _ => continue,
            };

            let node = local.entry(token.name).or_default();
            node.count += 1;
            node.as_in.push(oracle_text.clone());
        }

        let done_template = "Thread {prefix:>2} [{bar:40.green/white}] {pos:>6}/{len:6} ({percent}%)";
        let style = indicatif::ProgressStyle::with_template(done_template)
            .unwrap()
            .progress_chars("─● ");
        pb.set_style(style);
        pb.finish_with_message("done");

        // Single lock per thread: drain the local map into the shared one.
        let mut shared = most_required_nodes.lock().unwrap();
        for (key, local_node) in local {
            let entry = shared.entry(key).or_default();
            entry.count += local_node.count;
            entry.as_in.extend(local_node.as_in);
        }
    });

    let most_required_nodes = most_required_nodes.into_inner().unwrap();
    let mut most_required_tokens: Vec<_> = most_required_nodes.into_iter().collect();
    most_required_tokens.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

    const UNDERLINE_ON: &str = "\x1b[4m";
    const UNDERLINE_RESET: &str = "\x1b[24m";

    println!("");
    println!(
        "\nMost required nodes: (showing {SHOWN_NODES} out of {})",
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
