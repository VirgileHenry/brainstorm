mod another_specified;
mod attached;
mod count_specified;
mod one_among;
mod previously_mentionned;
mod self_referencing;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        another_specified::rules().collect::<Vec<_>>(),
        attached::rules().collect::<Vec<_>>(),
        count_specified::rules().collect::<Vec<_>>(),
        one_among::rules().collect::<Vec<_>>(),
        previously_mentionned::rules().collect::<Vec<_>>(),
        self_referencing::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
