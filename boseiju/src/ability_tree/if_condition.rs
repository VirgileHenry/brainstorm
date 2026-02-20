use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Clauses that describes an "if" statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum IfCondition {
    EventOccured(IfConditionEventOccured),
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
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "if condition:")?;
        out.push_final_branch()?;
        match self {
            Self::EventOccured(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct IfConditionEventOccured {
    event: crate::ability_tree::event::Event,
    timeframe: crate::ability_tree::time::BackwardDuration,
}

impl crate::ability_tree::AbilityTreeNode for IfConditionEventOccured {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::IfConditionEventOccured.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.event as &dyn AbilityTreeNode);
        children.push(&self.timeframe as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event occured:")?;
        out.push_inter_branch()?;
        write!(out, "event:")?;
        out.push_final_branch()?;
        self.event.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "time frame:")?;
        out.push_final_branch()?;
        self.timeframe.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
