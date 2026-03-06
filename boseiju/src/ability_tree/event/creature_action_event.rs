mod creature_attacks_action;
mod creature_deals_combat_damage_action;
mod creature_dies_action;

pub use creature_attacks_action::CreatureAttacksAction;
pub use creature_deals_combat_damage_action::CreatureDealsCombatDamageAction;
pub use creature_dies_action::CreatureDiesAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Events related to creatures.
///
/// These events are grouped together, but maybe they should not ?
/// From a human perspective, it makes it easier to classify them this way.
///
/// Creature events includes attacking, dealing damages, blocking, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureActionEvent {
    pub creatures: crate::ability_tree::object::ObjectReference,
    pub action: CreatureAction,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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
        out.next_final_branch()?;
        write!(out, "action:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature action event"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureActionEvent {
    fn dummy_init() -> Self {
        Self {
            creatures: crate::utils::dummy(),
            action: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureAction {
    Attacks(CreatureAttacksAction),
    DealsCombatDamage(CreatureDealsCombatDamageAction),
    Dies(CreatureDiesAction),
}

#[cfg(feature = "spanned_tree")]
impl CreatureAction {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacks(child) => child.span,
            Self::DealsCombatDamage(child) => child.span,
            Self::Dies(child) => child.span,
        }
    }
}

impl AbilityTreeNode for CreatureAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn AbilityTreeNode),
            Self::DealsCombatDamage(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Dies(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature action")?;
        out.push_final_branch()?;
        match self {
            Self::Attacks(action) => action.display(out)?,
            Self::DealsCombatDamage(action) => action.display(out)?,
            Self::Dies(action) => action.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature action"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacks(child) => child.node_span(),
            Self::DealsCombatDamage(child) => child.node_span(),
            Self::Dies(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureAction {
    fn dummy_init() -> Self {
        Self::DealsCombatDamage(crate::utils::dummy())
    }
}
