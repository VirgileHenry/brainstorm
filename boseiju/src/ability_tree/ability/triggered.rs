mod trigger_cond;

pub use trigger_cond::TriggerCondition;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A triggered ability is an ability that waits for an event to happen
/// to put an effect on the stack.
///
/// From the Comprehensive Rules:
/// A kind of ability. Triggered abilities begin with the word “when,” “whenever,” or “at.”
/// They’re written as “[Trigger condition], [effect].”
/// See rule 113, “Abilities,” and rule 603, “Handling Triggered Abilities.”
///
/// See also: https://mtg.fandom.com/wiki/Triggered_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TriggeredAbility {
    pub condition: TriggerCondition,
    pub effect: crate::ability_tree::ability::spell::SpellAbility,
}

impl AbilityTreeNode for TriggeredAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TriggeredAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn AbilityTreeNode);
        children.push(&self.condition as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "triggered ability:")?;
        out.push_inter_branch()?;
        write!(out, "condition:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
