mod planeswalker;
mod planeswalker_specifiers;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        planeswalker::rules().collect::<Vec<_>>(),
        planeswalker_specifiers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
