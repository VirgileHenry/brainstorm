use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PlayerSpecifier {
    All,
    Any,
    EachOpponent,
    TargetOpponent,
    ToYourLeft,
    ToYourRight,
    You,
}

impl AbilityTreeNode for PlayerSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::PlayerSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::PlayerSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl Terminal for PlayerSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "each player" => Some(Self::All),
            "an opponent" | "a player" => Some(Self::Any),
            "each opponent" | "opponents" | "your opponents" => Some(Self::EachOpponent),
            "target opponent" => Some(Self::TargetOpponent),
            "the player to your left" => Some(Self::ToYourLeft),
            "the player to your right" => Some(Self::ToYourRight),
            "you" => Some(Self::You),
            _ => None,
        }
    }
}

impl std::fmt::Display for PlayerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all players"),
            Self::Any => write!(f, "a player"),
            Self::EachOpponent => write!(f, "each opponent"),
            Self::TargetOpponent => write!(f, "target opponent"),
            Self::ToYourLeft => write!(f, "the player to your left"),
            Self::ToYourRight => write!(f, "the player to your right"),
            Self::You => write!(f, "you"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerSpecifier {
    fn dummy_init() -> Self {
        Self::You
    }
}
