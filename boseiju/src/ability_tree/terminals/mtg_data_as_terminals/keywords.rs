use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

use crate::ability_tree::NodeKind;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use idris::Idris;

impl Terminal for mtg_data::KeywordAction {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(source).ok()
    }
}

impl AbilityTreeNode for mtg_data::AbilityWord {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::AbilityWordIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::MtgData(MtgDataNodeKind::AbilityWord(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for mtg_data::AbilityWord {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(source).ok()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for mtg_data::AbilityWord {
    fn dummy_init() -> Self {
        Self::Landfall
    }
}
