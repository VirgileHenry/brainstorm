mod creature_attacks_action;
mod creature_blocks_action;
mod creature_deals_damage_action;
mod creature_dies_action;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature_attacks_action::rules().collect::<Vec<_>>(),
        creature_blocks_action::rules().collect::<Vec<_>>(),
        creature_deals_damage_action::rules().collect::<Vec<_>>(),
        creature_dies_action::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
