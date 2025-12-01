//! This main only attempts to parse all cards from the json and gives out the result.
//! The binary only exist for this for now

mod card;
mod cards;

pub use card::*;
pub use cards::AllCardsIter;

fn main() {
    let cards = AllCardsIter::new();
    let count = cards.len();

    println!("Parsed {count} cards!");
}
