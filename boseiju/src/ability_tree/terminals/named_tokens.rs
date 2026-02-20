use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum NamedToken {
    KomasCoil,
}

impl AbilityTreeNode for NamedToken {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::NamedTokenIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedToken(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for NamedToken {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "koma's coil" => Some(Self::KomasCoil),
            "~'s coil" => Some(Self::KomasCoil), /* Weird case for koma creating koma's coil */
            _ => None,
        }
    }
}

impl std::fmt::Display for NamedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KomasCoil => write!(f, "koma's coil"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for NamedToken {
    fn dummy_init() -> Self {
        Self::KomasCoil
    }
}
