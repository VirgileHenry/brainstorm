mod creature;
mod creature_specifiers;
mod specifiers_creature;
mod specifiers_creature_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature::rules().collect::<Vec<_>>(),
        creature_specifiers::rules().collect::<Vec<_>>(),
        specifiers_creature::rules().collect::<Vec<_>>(),
        specifiers_creature_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
