use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_DAMAGES_DEALT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Imperative to deal damage to things.
///
/// The imperative can deals multiple damages to multiple targets,
/// for instance Drakuseth, Maw of Flames states: "it deals 4 damage to any
/// target and 3 damage to each of up to two other targets."
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct DealsDamageImperative {
    pub dealer: crate::ability_tree::object::ObjectReference,
    pub damages: crate::utils::HeapArrayVec<DamagesDealt, MAX_DAMAGES_DEALT>,
}

impl AbilityTreeNode for DealsDamageImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DealsDamageImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.dealer as &dyn AbilityTreeNode);
        for damage in self.damages.iter() {
            children.push(damage as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deals damage:")?;
        out.push_inter_branch()?;
        write!(out, "dealer:")?;
        out.push_final_branch()?;
        self.dealer.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "damages dealt:")?;
        for (i, damage) in self.damages.iter().enumerate() {
            if i == self.damages.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            damage.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for DealsDamageImperative {
    fn dummy_init() -> Self {
        Self {
            dealer: crate::utils::dummy(),
            damages: crate::utils::HeapArrayVec::new(),
        }
    }
}

/// Single damage dealing action.
///
/// This can be the same damage to multiple targets, or shared damage among targets,
/// or anything that was mentionned in a single damage dealing action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct DamagesDealt {
    pub to: crate::ability_tree::object::ObjectReference,
    pub amount: crate::ability_tree::number::Number,
}

impl AbilityTreeNode for DamagesDealt {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DamagesDealt.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.to as &dyn AbilityTreeNode);
        children.push(&self.amount as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "damage dealt:")?;
        out.push_inter_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for DamagesDealt {
    fn dummy_init() -> Self {
        Self {
            to: crate::utils::dummy(),
            amount: crate::utils::dummy(),
        }
    }
}
