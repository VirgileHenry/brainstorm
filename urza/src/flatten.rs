use crate::node_id::NodeId;

pub struct NodeRepr<const PAYLOAD_SIZE: usize> {
    pub node_id: u32,
    pub payload: [f32; PAYLOAD_SIZE],
}

/// The flatten trait allows to transform basic types into
/// vector representation we can feed our neural network.
pub trait Flatten<const FLAT_SIZE: usize> {
    fn flatten(&self) -> NodeRepr<FLAT_SIZE>;
}

impl Flatten<5> for mtg_data::Color {
    fn flatten(&self) -> NodeRepr<5> {
        let node_id = self.node_id();
        let payload = match self {
            mtg_data::Color::Colorless => [0.0, 0.0, 0.0, 0.0, 0.0],
            mtg_data::Color::White => [1.0, 0.0, 0.0, 0.0, 0.0],
            mtg_data::Color::Blue => [0.0, 1.0, 0.0, 0.0, 0.0],
            mtg_data::Color::Black => [0.0, 0.0, 1.0, 0.0, 0.0],
            mtg_data::Color::Red => [0.0, 0.0, 0.0, 1.0, 0.0],
            mtg_data::Color::Green => [0.0, 0.0, 0.0, 0.0, 1.0],
        };
        NodeRepr { node_id, payload }
    }
}
