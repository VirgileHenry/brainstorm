mod player_attacks_action;
mod player_casts_spell_action;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        player_attacks_action::rules().collect::<Vec<_>>(),
        player_casts_spell_action::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
