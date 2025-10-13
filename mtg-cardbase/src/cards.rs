/// If this throws an error, you might be missing the card database.
/// Run the python script "data_fetcher.py" to get it.
const CARDS_JSON: &'static str = include_str!("../data/cards.json");

pub struct AllCardsIter(std::collections::VecDeque<crate::Card>);

impl AllCardsIter {
    pub fn new() -> Self {
        let json_obj =
            crate::static_json::parse(CARDS_JSON).expect("Invalid json for provided card list!");
        let json_vec = match json_obj {
            crate::static_json::StaticJsonValue::Array(array) => array,
            other => panic!(
                "Invalid json for provided cards list! Expected array, found {:?}",
                other
            ),
        };
        use crate::static_json::FromJsonValue;
        let all_cards = json_vec
            .into_iter()
            .map(|json_card| crate::Card::from_json_value(&json_card))
            .collect::<Result<_, _>>()
            .expect("Failed to convert a card from json");
        AllCardsIter(all_cards)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Iterator for AllCardsIter {
    type Item = crate::Card;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
