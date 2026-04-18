mod add_mana_rules;
mod change_zone_rules;
mod choose_rules;
mod create_token_rules;
mod deals_damage_rules;
mod destroy_rules;
mod discard_rules;
mod draw_rules;
mod exile_follow_up_rules;
mod exile_rules;
mod gain_life_rules;
mod generate_continuous_effect_rules;
mod generate_delayed_triggered_ab_rules;
mod put_counters_rules;
mod remove_counters_rules;
mod sacrifice_rules;
mod tap_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        add_mana_rules::rules().collect::<Vec<_>>(),
        choose_rules::rules().collect::<Vec<_>>(),
        create_token_rules::rules().collect::<Vec<_>>(),
        deals_damage_rules::rules().collect::<Vec<_>>(),
        destroy_rules::rules().collect::<Vec<_>>(),
        discard_rules::rules().collect::<Vec<_>>(),
        draw_rules::rules().collect::<Vec<_>>(),
        exile_follow_up_rules::rules().collect::<Vec<_>>(),
        exile_rules::rules().collect::<Vec<_>>(),
        gain_life_rules::rules().collect::<Vec<_>>(),
        generate_continuous_effect_rules::rules().collect::<Vec<_>>(),
        generate_delayed_triggered_ab_rules::rules().collect::<Vec<_>>(),
        put_counters_rules::rules().collect::<Vec<_>>(),
        remove_counters_rules::rules().collect::<Vec<_>>(),
        change_zone_rules::rules().collect::<Vec<_>>(),
        sacrifice_rules::rules().collect::<Vec<_>>(),
        tap_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
