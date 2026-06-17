mod conditional_imperative;
mod imperatives;
mod may_ability;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        imperatives::rules().collect::<Vec<_>>(),
        may_ability::rules().collect::<Vec<_>>(),
        conditional_imperative::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
