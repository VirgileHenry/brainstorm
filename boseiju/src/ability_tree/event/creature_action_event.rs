mod creature_deals_combat_damage_action;

pub use creature_deals_combat_damage_action::CreatureDealsCombatDamageAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Events related to creatures.
///
/// These events are grouped together, but maybe they should not ?
/// From a human perspective, it makes it easier to classify them this way.
///
/// Creature events includes attacking, dealing damages, blocking, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CreatureActionEvent {
    pub creatures: crate::ability_tree::object::ObjectReference,
    pub action: CreatureAction,
}

impl AbilityTreeNode for CreatureActionEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureActionEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creatures as &dyn AbilityTreeNode);
        children.push(&self.action as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature action")?;
        out.push_inter_branch()?;
        write!(out, "creatures:")?;
        out.push_final_branch()?;
        self.creatures.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "action:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureActionEvent {
    fn dummy_init() -> Self {
        Self {
            creatures: crate::utils::dummy(),
            action: crate::utils::dummy(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CreatureAction {
    DealsCombatDamage(CreatureDealsCombatDamageAction),
}

impl AbilityTreeNode for CreatureAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::DealsCombatDamage(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature action")?;
        out.push_inter_branch()?;
        match self {
            Self::DealsCombatDamage(action) => action.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureAction {
    fn dummy_init() -> Self {
        Self::DealsCombatDamage(crate::utils::dummy())
    }
}
