mod creature_match_specifier;
mod event_occured;
mod player_control_permanent;
mod stack_object_has_state;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature_match_specifier::rules().collect::<Vec<_>>(),
        event_occured::rules().collect::<Vec<_>>(),
        player_control_permanent::rules().collect::<Vec<_>>(),
        stack_object_has_state::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
