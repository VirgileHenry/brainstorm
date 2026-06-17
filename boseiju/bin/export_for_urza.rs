//! Attempt to parse all cards, and keep the 100 with the most words that were parsed.
//!
//! These are kept as examples card in the web demo.
use boseiju::*;
use rayon::prelude::*;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let cards = mtg_cardbase::AllCardsIter::hexxed_v1_cards();
    const OUTPUT_FILE: &'static str = "boseiju_export.json";

    let cards_vec: Vec<_> = cards.iter().collect();
    let num_threads = rayon::current_num_threads();
    let chunk_size = cards_vec.len().div_ceil(num_threads);

    let template = "Thread {prefix:>2} [{bar:40.yellow/white}] {pos:>6}/{len:6} ({percent}%)";
    let style = indicatif::ProgressStyle::with_template(template)
        .unwrap()
        .progress_chars("─● ");

    let multi = indicatif::MultiProgress::new();
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

    let result: Vec<Card> = chunks
        .par_iter()
        .zip(bars.par_iter())
        .flat_map(|(chunk, pb)| {
            let total = chunk.len();
            let mut last_shown_percentage = 0usize;
            let mut local: Vec<Card> = Vec::with_capacity(total);

            for (i, card) in chunk.iter().enumerate() {
                let progress = (i + 1) * 100 / total;
                if progress != last_shown_percentage {
                    pb.set_position(((i + 1) as u64).min(total as u64));
                    last_shown_percentage = progress;
                }

                if let Ok(parsed) = Card::try_from(*card) {
                    local.push(parsed);
                }
            }

            let done_template = "Thread {prefix:>2} [{bar:40.green/white}] {pos:>6}/{len:6} ({percent}%)";
            let style = indicatif::ProgressStyle::with_template(done_template)
                .unwrap()
                .progress_chars("─● ");
            pb.set_style(style);
            pb.finish_with_message("done");
            local
        })
        .collect();

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
