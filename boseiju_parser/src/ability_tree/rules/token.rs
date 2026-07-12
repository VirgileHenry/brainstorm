mod custom_tokens;
mod known_tokens;
mod token_type_line;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        custom_tokens::rules().collect::<Vec<_>>(),
        known_tokens::rules().collect::<Vec<_>>(),
        token_type_line::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
