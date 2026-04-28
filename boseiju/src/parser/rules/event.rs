mod creature_performs_action;
mod object_gains_state;
mod permanent_performs_action;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature_performs_action::rules().collect::<Vec<_>>(),
        object_gains_state::rules().collect::<Vec<_>>(),
        permanent_performs_action::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
