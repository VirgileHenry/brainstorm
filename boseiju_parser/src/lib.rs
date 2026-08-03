mod ability_tree;
mod card;

pub use ability_tree::ParserError;
pub use ability_tree::ParserNode;
pub use card::CardParseError;
pub use card::LayoutParseError;

/// Entry point for parsing an ability tree.
///
/// Attempts to parse a sequence of nodes into an ability tree, using the Earley parsing algorithm.
///
/// The algorithm reference can be found here: <https://en.wikipedia.org/wiki/Earley_parser>
/// The algorithm used for the implementation was: <https://fr.wikipedia.org/wiki/Analyse_Earley> (cocorico)
pub fn parse_ability_tree(tokens: &[boseiju_lexer::Token]) -> Result<boseiju_tree::AbilityTree, ability_tree::ParserError> {
    ability_tree::parse_impl(tokens)
}

/// Entry point to parse an entire card from json (scryfall format) to a fully structured MTG card (boseiju format).
pub fn parse_card(card: &mtg_cardbase::Card) -> Result<boseiju_tree::Card, card::CardParseError> {
    <boseiju_tree::card::Card as card::Parse>::parse(card)
}
