/// The flatten trait allows to transform basic types into
/// vector representation we can feed our neural network.
pub trait Flatten<const FLAT_SIZE: usize> {
    fn flatten(&self) -> [f32; FLAT_SIZE];
}

impl Flatten<512> for tolaria::Card {
    fn flatten(&self) -> [f32; 512] {
        [0.0; 512]
    }
}

impl Flatten<5> for mtg_data::Color {
    fn flatten(&self) -> [f32; 5] {
        match self {
            mtg_data::Color::Colorless => [0.0, 0.0, 0.0, 0.0, 0.0],
            mtg_data::Color::White => [1.0, 0.0, 0.0, 0.0, 0.0],
            mtg_data::Color::Blue => [0.0, 1.0, 0.0, 0.0, 0.0],
            mtg_data::Color::Black => [0.0, 0.0, 1.0, 0.0, 0.0],
            mtg_data::Color::Red => [0.0, 0.0, 0.0, 1.0, 0.0],
            mtg_data::Color::Green => [0.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}
