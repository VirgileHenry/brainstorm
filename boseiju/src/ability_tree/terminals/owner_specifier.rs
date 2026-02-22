use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum OwnerSpecifier {
    YouOwn,
    YouDontOwn,
    ObjectOwner,
}

impl AbilityTreeNode for OwnerSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::OwnerSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::OwnerSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for OwnerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerSpecifier::YouOwn => write!(f, "you own"),
            OwnerSpecifier::YouDontOwn => write!(f, "you don't own"),
            OwnerSpecifier::ObjectOwner => write!(f, "its owner"),
        }
    }
}

impl Terminal for OwnerSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you own" | "your" => Some(OwnerSpecifier::YouOwn),
            "you don't own" => Some(OwnerSpecifier::YouDontOwn),
            "its owner" => Some(OwnerSpecifier::ObjectOwner),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnerSpecifier {
    fn dummy_init() -> Self {
        Self::YouOwn
    }
}
