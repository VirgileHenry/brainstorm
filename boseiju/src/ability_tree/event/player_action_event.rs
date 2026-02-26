mod player_attacks_action;

pub use player_attacks_action::PlayerAttacksAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Events related to Player foing actions.
///
/// Player events includes attacking, drawing cards, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlayerActionEvent {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub action: PlayerAction,
}

impl AbilityTreeNode for PlayerActionEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerActionEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        children.push(&self.action as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player action")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "action:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerActionEvent {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            action: crate::utils::dummy(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PlayerAction {
    Attacks(PlayerAttacksAction),
}

impl AbilityTreeNode for PlayerAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player action")?;
        out.push_final_branch()?;
        match self {
            Self::Attacks(action) => action.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerAction {
    fn dummy_init() -> Self {
        Self::Attacks(crate::utils::dummy())
    }
}
