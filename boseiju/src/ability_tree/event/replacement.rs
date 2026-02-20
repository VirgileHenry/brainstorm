mod counter_on_permanent;
mod source_ref;
mod token_creation;

pub use counter_on_permanent::CounterOnPermanentReplacement;
pub use source_ref::EventSourceReference;
pub use token_creation::TokenCreationReplacement;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum EventReplacement {
    TokenCreation(TokenCreationReplacement),
    CounterOnPermanent(CounterOnPermanentReplacement),
}

impl crate::ability_tree::AbilityTreeNode for EventReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EventReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::TokenCreation(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CounterOnPermanent(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event replacement")?;
        out.push_final_branch()?;
        match self {
            Self::TokenCreation(child) => child.display(out)?,
            Self::CounterOnPermanent(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
