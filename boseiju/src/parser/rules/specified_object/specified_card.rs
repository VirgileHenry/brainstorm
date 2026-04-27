mod count_card;
mod count_specifiers_card;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        count_card::rules().collect::<Vec<_>>(),
        count_specifiers_card::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
