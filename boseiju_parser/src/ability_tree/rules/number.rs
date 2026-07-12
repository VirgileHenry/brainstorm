mod english_numbers;
mod game_state_numbers;
mod number_literals;
mod x_numbers;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        english_numbers::rules().collect::<Vec<_>>(),
        game_state_numbers::rules().collect::<Vec<_>>(),
        number_literals::rules().collect::<Vec<_>>(),
        x_numbers::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
