mod choose_rules;
mod deals_damage_rules;
mod destroy_rules;
mod discard_rules;
mod draw_rules;
mod exile_follow_up_rules;
mod exile_rules;
mod gain_life_rules;
mod put_counters_rules;
mod remove_counters_rules;
mod return_rules;
mod sacrifice_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        choose_rules::rules().collect::<Vec<_>>(),
        deals_damage_rules::rules().collect::<Vec<_>>(),
        destroy_rules::rules().collect::<Vec<_>>(),
        discard_rules::rules().collect::<Vec<_>>(),
        draw_rules::rules().collect::<Vec<_>>(),
        exile_follow_up_rules::rules().collect::<Vec<_>>(),
        exile_rules::rules().collect::<Vec<_>>(),
        gain_life_rules::rules().collect::<Vec<_>>(),
        put_counters_rules::rules().collect::<Vec<_>>(),
        remove_counters_rules::rules().collect::<Vec<_>>(),
        return_rules::rules().collect::<Vec<_>>(),
        sacrifice_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
