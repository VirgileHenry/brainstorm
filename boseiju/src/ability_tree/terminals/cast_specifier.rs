use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::terminals::Terminal;

/// A specifier for who casts a spell.
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CastSpecifier {
    YouCast,
    YourOpponentsCast,
}

impl AbilityTreeNode for CastSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::CastSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::CastSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for CastSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastSpecifier::YouCast => write!(f, "you cast"),
            CastSpecifier::YourOpponentsCast => write!(f, "your opponents cast"),
        }
    }
}

impl Terminal for CastSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_str(_: &str) -> Option<Self> {
        /*
         * "you cast" can't be lexed directyly, as it may not be a cast specifier.
         * For instance, "whenever you cast a spell" is not the same meaning as
         * "spells you cast cost 1 less to cast".
         */
        None
    }
}
