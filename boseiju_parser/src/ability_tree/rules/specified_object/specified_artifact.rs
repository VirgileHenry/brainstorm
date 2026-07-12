mod artifact;
mod artifact_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        artifact::rules().collect::<Vec<_>>(),
        artifact_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
