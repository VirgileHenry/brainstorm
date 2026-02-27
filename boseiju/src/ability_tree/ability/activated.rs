use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_COST_COUNT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ActivatedAbility {
    effect: crate::ability_tree::ability::spell::SpellAbility,
    costs: crate::utils::HeapArrayVec<crate::ability_tree::cost::Cost, MAX_COST_COUNT>,
}

impl AbilityTreeNode for ActivatedAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ActivatedAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn AbilityTreeNode);
        for cost in self.costs.iter() {
            children.push(cost as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "activated ability:")?;
        out.push_inter_branch()?;
        write!(out, "costs:")?;
        out.push_final_branch()?;
        for cost in self.costs.iter().take(self.costs.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            cost.display(out)?;
            out.pop_branch();
        }
        if let Some(cost) = self.costs.last() {
            out.push_inter_branch()?;
            cost.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "effects:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
