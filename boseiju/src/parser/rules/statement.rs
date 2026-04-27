mod imperatives;
mod may_ability;
mod replacable_imperative;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        imperatives::rules().collect::<Vec<_>>(),
        may_ability::rules().collect::<Vec<_>>(),
        replacable_imperative::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
