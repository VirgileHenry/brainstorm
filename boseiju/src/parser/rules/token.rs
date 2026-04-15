mod custom_tokens;
mod known_tokens;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        custom_tokens::rules().collect::<Vec<_>>(),
        known_tokens::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
