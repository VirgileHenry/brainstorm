mod artifact_specifiers;
mod card_specifiers;
mod common_specifiers;
mod creature_specifiers;
mod enchantment_specifiers;
mod land_specifiers;
mod permanent_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        artifact_specifiers::rules().collect::<Vec<_>>(),
        card_specifiers::rules().collect::<Vec<_>>(),
        common_specifiers::rules().collect::<Vec<_>>(),
        creature_specifiers::rules().collect::<Vec<_>>(),
        enchantment_specifiers::rules().collect::<Vec<_>>(),
        land_specifiers::rules().collect::<Vec<_>>(),
        permanent_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
