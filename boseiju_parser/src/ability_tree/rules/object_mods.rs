mod creature_get_p_t;
mod creature_get_p_t_and_has_ab;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        creature_get_p_t::rules().collect::<Vec<_>>(),
        creature_get_p_t_and_has_ab::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
