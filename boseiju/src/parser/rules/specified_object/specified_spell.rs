mod specifiers_spell;
mod spell;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        specifiers_spell::rules().collect::<Vec<_>>(),
        spell::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
