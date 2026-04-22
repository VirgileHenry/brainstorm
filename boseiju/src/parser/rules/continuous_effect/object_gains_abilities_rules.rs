mod object_gain_ab_until;
mod object_get_p_t;
mod object_get_p_t_and_has_ab;
mod object_get_x_x;
mod object_have_ab;

use crate::parser::rules::ParserRule;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    [
        object_get_x_x::rules().collect::<Vec<_>>(),
        object_gain_ab_until::rules().collect::<Vec<_>>(),
        object_get_p_t::rules().collect::<Vec<_>>(),
        object_get_p_t_and_has_ab::rules().collect::<Vec<_>>(),
        object_have_ab::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
