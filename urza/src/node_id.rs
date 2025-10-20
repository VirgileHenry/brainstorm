pub trait NodeId {
    fn node_id(&self) -> u32;
}

impl NodeId for mtg_data::Color {
    fn node_id(&self) -> u32 {
        0
    }
}
