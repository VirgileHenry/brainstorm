mod creature_performs_action;
mod permanent_performs_action;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        creature_performs_action::rules().collect::<Vec<_>>(),
        permanent_performs_action::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
