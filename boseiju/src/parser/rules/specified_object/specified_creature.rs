mod another_creature;
mod another_creature_specifiers;
mod count_creature;
mod count_creature_specifiers;
mod count_specifiers_creature;
mod count_specifiers_creature_specifiers;
mod creature_specifiers;
mod specifiers_creature;
mod specifiers_creature_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        another_creature::rules().collect::<Vec<_>>(),
        another_creature_specifiers::rules().collect::<Vec<_>>(),
        count_creature::rules().collect::<Vec<_>>(),
        count_creature_specifiers::rules().collect::<Vec<_>>(),
        count_specifiers_creature::rules().collect::<Vec<_>>(),
        count_specifiers_creature_specifiers::rules().collect::<Vec<_>>(),
        creature_specifiers::rules().collect::<Vec<_>>(),
        specifiers_creature::rules().collect::<Vec<_>>(),
        specifiers_creature_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
