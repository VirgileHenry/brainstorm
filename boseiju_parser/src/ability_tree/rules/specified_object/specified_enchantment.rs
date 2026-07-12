mod enchantment;
mod enchantment_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        enchantment::rules().collect::<Vec<_>>(),
        enchantment_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
