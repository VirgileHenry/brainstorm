use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::ability_tree::terminals::Terminal;

impl AbilityTreeNode for mtg_data::Color {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::Color).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        mtg_data::Color::all()
            .map(|color| if color == *self { 1 } else { 0 })
            .collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::Color {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        use std::str::FromStr;
        mtg_data::Color::from_str(source).ok()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::Color {
    fn dummy_init() -> Self {
        Self::Colorless
    }
}
