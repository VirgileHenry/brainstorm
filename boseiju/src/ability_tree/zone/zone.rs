use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum OwnableZone {
    Graveyard,
    Hand,
    Library,
}

impl AbilityTreeNode for OwnableZone {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::OwnableZoneIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::OwnableZone(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for OwnableZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnableZone::Graveyard => write!(f, "graveyard"),
            OwnableZone::Hand => write!(f, "hand"),
            OwnableZone::Library => write!(f, "library"),
        }
    }
}

impl Terminal for OwnableZone {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "graveyard" => Some(OwnableZone::Graveyard),
            "hand" => Some(OwnableZone::Hand),
            "library" => Some(OwnableZone::Library),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnableZone {
    fn dummy_init() -> Self {
        Self::Library
    }
}
