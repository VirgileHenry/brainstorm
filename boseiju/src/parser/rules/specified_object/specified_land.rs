mod land;
mod land_specifiers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        land::rules().collect::<Vec<_>>(),
        land_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
