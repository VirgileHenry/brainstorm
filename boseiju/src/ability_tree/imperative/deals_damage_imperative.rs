use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct DealsDamageImperative {
    pub dealer: crate::ability_tree::object::ObjectReference,
    pub amount: crate::ability_tree::number::Number,
    pub to: crate::ability_tree::object::ObjectReference,
}

impl AbilityTreeNode for DealsDamageImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DealsDamageImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.dealer as &dyn AbilityTreeNode);
        children.push(&self.amount as &dyn AbilityTreeNode);
        children.push(&self.to as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deals damage:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.dealer.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
