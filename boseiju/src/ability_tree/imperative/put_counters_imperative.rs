use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PutCountersImperative {
    pub amount: crate::ability_tree::number::Number,
    pub of: crate::ability_tree::terminals::Counter,
    pub on: crate::ability_tree::object::ObjectReference,
}

impl AbilityTreeNode for PutCountersImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PutCountersImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn AbilityTreeNode);
        children.push(&self.of as &dyn AbilityTreeNode);
        children.push(&self.on as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put:")?;
        out.push_inter_branch()?;
        write!(out, "number:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "of:")?;
        out.push_final_branch()?;
        write!(out, "{}", self.of)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "on:")?;
        out.push_final_branch()?;
        self.on.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
