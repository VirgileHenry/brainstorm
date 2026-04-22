mod event_cant_happen;
mod object_gains_abilities_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        event_cant_happen::rules().collect::<Vec<_>>(),
        object_gains_abilities_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
