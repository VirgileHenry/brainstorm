use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CountSpecifier {
    All,
    Target,
    UpTo { up_to: u32 },
    AnyNumberOfTargets,
    Others,
}

impl AbilityTreeNode for CountSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::CountSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::CountSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for CountSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "all" => Some(Self::All),
            "each" => Some(Self::All),
            "target" | "targets" => Some(Self::Target),
            "any target" => Some(Self::Target),
            "any number of target" | "any number of targets" => Some(Self::AnyNumberOfTargets),
            "other" | "others" => Some(Self::Others),
            other => {
                let prefix = "up to ";
                if other.starts_with(prefix) {
                    let num = crate::utils::parse_num(&other[prefix.len()..])?;
                    Some(Self::UpTo { up_to: num })
                } else {
                    None
                }
            }
        }
    }
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Target => write!(f, "target"),
            Self::UpTo { up_to: num } => write!(f, "up to {num}"),
            Self::AnyNumberOfTargets => write!(f, "any number of target"),
            Self::Others => write!(f, "others"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CountSpecifier {
    fn dummy_init() -> Self {
        Self::You
    }
}
