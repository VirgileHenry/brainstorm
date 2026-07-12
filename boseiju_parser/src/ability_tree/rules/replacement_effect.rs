mod etb_replacement;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [etb_replacement::rules().collect::<Vec<_>>()].into_iter().flatten()
}
