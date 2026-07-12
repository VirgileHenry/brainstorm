mod modify_rules_effects;
mod object_gains_abilities_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        modify_rules_effects::rules().collect::<Vec<_>>(),
        object_gains_abilities_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
