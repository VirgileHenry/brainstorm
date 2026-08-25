mod atomic_cost;
mod cost_grouping;
mod imperative_cost;
mod life_cost;
mod mana_cost;
mod tap_cost;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        atomic_cost::rules().collect::<Vec<_>>(),
        cost_grouping::rules().collect::<Vec<_>>(),
        imperative_cost::rules().collect::<Vec<_>>(),
        life_cost::rules().collect::<Vec<_>>(),
        mana_cost::rules().collect::<Vec<_>>(),
        tap_cost::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
