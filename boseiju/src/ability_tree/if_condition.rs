mod event_occured;
mod this_is_your_turn;

pub use event_occured::IfConditionEventOccured;
pub use this_is_your_turn::IfConditionThisIsYourTurn;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Clauses that describes an "if" statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum IfCondition {
    EventOccured(IfConditionEventOccured),
    ThisIsYourTurn(IfConditionThisIsYourTurn),
}

impl AbilityTreeNode for IfCondition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::IfCondition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::EventOccured(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ThisIsYourTurn(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "if condition:")?;
        out.push_final_branch()?;
        match self {
            Self::EventOccured(child) => child.display(out)?,
            Self::ThisIsYourTurn(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for IfCondition {
    fn dummy_init() -> Self {
        Self::ThisIsYourTurn(crate::utils::dummy())
    }
}
