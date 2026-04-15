use std::ops::{Deref, DerefMut};

pub struct AllCardsIter(Vec<crate::Card>);

impl AllCardsIter {
    pub fn new() -> Self {
        /* All possible serach paths ? */
        const SEARCH_PATHS: &[&'static str] = &[
            /* Cargo bin run the executables from the workspaces */
            "mtg-cardbase/data/cards.json",
            /* Cargo test run them from the crates, so we need to go back */
            "../mtg-cardbase/data/cards.json",
            /* Brainstorm can be used as a lib, so we shall account for that too */
            "../brainstorm/mtg-cardbase/data/cards.json",
        ];
        /*
            If this throws an error, you might be missing the card database.
            Run the python script "data_fetcher.py" to get it.
        */
        let mut cards_json = None;
        for search_path in SEARCH_PATHS {
            if let Ok(cards) = std::fs::read_to_string(search_path) {
                cards_json = Some(cards);
            }
        }
        let cards_json = cards_json.expect("Missing json card database!");
        let all_cards: Vec<crate::Card> = serde_json::from_str(&cards_json).expect("Invalid json for provided card list!");
        Self(all_cards)
    }

    pub fn standard_legal() -> Self {
        let cards = Self::new();
        let filter = |card: &crate::Card| card.legalities.standard == "legal";
        let standard_legal_cards = cards.0.into_iter().filter(filter);
        Self(standard_legal_cards.collect())
    }

    pub fn commander_legal() -> Self {
        let cards = Self::new();
        let filter = |card: &crate::Card| card.legalities.commander == "legal";
        let commander_legal_cards = cards.0.into_iter().filter(filter);
        Self(commander_legal_cards.collect())
    }

    pub fn hexxed_v1_cards() -> Self {
        let cards = Self::new();
        let filter = |card: &crate::Card| card.set_type != "funny" && card.set_type != "alchemy";
        let commander_legal_cards = cards.0.into_iter().filter(filter);
        Self(commander_legal_cards.collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Deref for AllCardsIter {
    type Target = [crate::Card];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AllCardsIter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
