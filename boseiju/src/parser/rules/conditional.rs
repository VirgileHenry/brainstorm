mod creature_match_specifier;
mod event_occured;
mod player_control_permanent;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature_match_specifier::rules().collect::<Vec<_>>(),
        event_occured::rules().collect::<Vec<_>>(),
        player_control_permanent::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
