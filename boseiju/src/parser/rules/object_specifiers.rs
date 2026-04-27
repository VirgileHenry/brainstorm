mod card_specifiers;
mod common_specifiers;
mod creature_specifiers;
mod permanent_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        card_specifiers::rules().collect::<Vec<_>>(),
        common_specifiers::rules().collect::<Vec<_>>(),
        creature_specifiers::rules().collect::<Vec<_>>(),
        permanent_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
