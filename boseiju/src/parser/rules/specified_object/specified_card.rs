mod card;
mod specifiers_card;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        card::rules().collect::<Vec<_>>(),
        specifiers_card::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
