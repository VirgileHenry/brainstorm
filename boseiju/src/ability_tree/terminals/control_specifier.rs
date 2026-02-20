use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Control specifier for objects.
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ControlSpecifier {
    YouControl,
    YouDontControl,
}

impl AbilityTreeNode for ControlSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::ControlSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::ControlSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for ControlSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YouControl => write!(f, "you control"),
            Self::YouDontControl => write!(f, "you don't control"),
        }
    }
}

impl Terminal for ControlSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you control" | "you already control" => Some(Self::YouControl),
            "you don't control" | "your opponents control" | "an opponent controls" => Some(Self::YouDontControl),
            _ => None,
        }
    }
}
