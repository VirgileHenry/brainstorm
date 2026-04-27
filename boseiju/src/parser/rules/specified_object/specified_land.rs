mod count_land;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [count_land::rules().collect::<Vec<_>>()].into_iter().flatten()
}
