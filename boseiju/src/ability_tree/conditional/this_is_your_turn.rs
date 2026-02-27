use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// If condition for event that only applies during your turn.
///
/// This condition will mostly appear silently with sentences like "when X during your turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ConditionThisIsYourTurn;

impl crate::ability_tree::AbilityTreeNode for ConditionThisIsYourTurn {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ThisIsYourTurn.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "this is your turn")
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionThisIsYourTurn {
    fn dummy_init() -> Self {
        Self
    }
}
