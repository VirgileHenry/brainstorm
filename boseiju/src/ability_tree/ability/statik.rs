pub mod continuous_effect;
pub mod cost_modification_effect;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A static ability, from the comprehensive rules:
///
/// A kind of ability.
/// Static abilities do something all the time rather than being activated or triggered.
/// See rule 113, “Abilities”, and rule 604, “Handling Static Abilities”.
///
/// See the MTG wiki: https://mtg.fandom.com/wiki/Static_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum StaticAbility {
    ContinuousEffect(continuous_effect::ContinuousEffect),
    CostModificationEffect(cost_modification_effect::CostModificationEffect),
}

impl AbilityTreeNode for StaticAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StaticAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ContinuousEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CostModificationEffect(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability:")?;
        out.push_final_branch()?;
        match self {
            Self::ContinuousEffect(ability) => ability.display(out)?,
            Self::CostModificationEffect(ability) => ability.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
