use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ObjectAttachedTo;

impl AbilityTreeNode for ObjectAttachedTo {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::ObjectAttachedTo).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for ObjectAttachedTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "object attached to")
    }
}

impl Terminal for ObjectAttachedTo {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "enchanted permanent" => Some(ObjectAttachedTo),
            "enchanted creature" => Some(ObjectAttachedTo),
            "enchanted artifact" => Some(ObjectAttachedTo),
            "enchanted land" => Some(ObjectAttachedTo),
            "enchanted planeswalker" => Some(ObjectAttachedTo),
            /* Fucking spellweaver volute */
            "enchanted instant card" => Some(ObjectAttachedTo),
            _ => None,
        }
    }
}
