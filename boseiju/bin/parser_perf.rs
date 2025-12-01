//! This test is used while developing the parser to get a rough estimate of what is fast.
//! I'll note here the results. Results mostly are coherent when tested with the same number of rules.
//! What we measure it the number of parser iter, which iw how many times the "fuse" func is called.
//! The lesser the better! So obviously we will attempt to get everything down
//! The format of the result will be min / median / average / max
//! ==== this line marks a change in the number of rules, meaning we increased the load.
//! Full brute force, with memoization: 0 / 1485 / 97_421 / 10_117_062 : very bad worst time
//! ==== Removed the terminals in the nodes, use the lexer directly, same algo:
//! Full brute force, with memoization: 0 / 702 / 45_169 / 5_747_436 : halfed the iter number
//! ==== Adding more rules, same algo:
//! Full brute force, with memoization: 0 / 3_302 / 197_197 / 16_207_344 : halfed the iter number
//! Iterative graph search: 0 / 840 / 5_756 / 273_248 : waaayyy better, but lost some parsing, needs fixing
//! Iterative graph search + memoization: 0 / 182 / 245 / 1_458 : insane, but lost everything sooo
//! Here, we started counting the successeful fuse and not the total fuse count. This is worst ?

use boseiju::*;

fn main() -> Result<(), String> {
    let cards = mtg_cardbase::AllCardsIter::new();

    // skip rayon for perf debugging, it messes the flamegraph
    // use rayon::iter::IntoParallelRefIterator;
    // use rayon::iter::ParallelIterator;
    let results: Vec<_> = cards
        .iter()
        .filter(|card| card.set == "fdn")
        .map(|card| match card.oracle_text.as_ref() {
            Some(text) => {
                let oracle_text = lexer::preprocess(&card.name, text);
                match lexer::lex(&oracle_text) {
                    /* Don't take into account cards we couldn't lex */
                    Err(_) => None,
                    Ok(tokens) => {
                        let (_, iters) = parser::parse_and_count_iters(&tokens);
                        Some((iters, card.name.as_str()))
                    }
                }
            }
            None => None,
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    let best = results
        .iter()
        .min_by(|(it1, _), (it2, _)| it1.cmp(it2))
        .unwrap_or(&(0, "Unknown"))
        .1;
    let worst = results
        .iter()
        .max_by(|(it1, _), (it2, _)| it1.cmp(it2))
        .unwrap_or(&(0, "Unknown"))
        .1;

    let mut results: Vec<_> = results.into_iter().map(|(it, _)| it).collect();

    results.sort();
    let median = results[results.len() / 2];
    let average = results.iter().cloned().sum::<usize>() / results.len();

    println!("Best case:    {:>16} ({best})", num_fmt(results[0]));
    println!("Median:       {:>16}", num_fmt(median));
    println!("Average:      {:>16}", num_fmt(average));
    println!("Worst case:   {:>16} ({worst})", num_fmt(results[results.len() - 1]));

    Ok(())
}

fn num_fmt(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push('_');
        }
        out.push(c);
    }
    out
}
