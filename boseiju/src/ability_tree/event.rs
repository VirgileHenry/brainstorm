pub mod replacement;
pub mod source;

mod create_token_event;
mod creature_action_event;
mod enters_the_battlefield_event;
mod life_gained_event;
mod player_action_event;
mod put_counter_on_permanent_event;

pub use create_token_event::CreateTokensEvent;
pub use creature_action_event::CreatureAction;
pub use creature_action_event::CreatureActionEvent;
pub use creature_action_event::CreatureAttacksAction;
pub use creature_action_event::CreatureDealsCombatDamageAction;
pub use creature_action_event::CreatureDiesAction;
pub use enters_the_battlefield_event::EntersTheBattlefieldEvent;
pub use life_gained_event::LifeGainedEvent;
pub use player_action_event::PlayerAction;
pub use player_action_event::PlayerActionEvent;
pub use player_action_event::PlayerAttacksAction;
pub use player_action_event::PlayerCastsSpellEvent;
pub use put_counter_on_permanent_event::PutCounterOnPermanentEvent;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event is anything that happens in a Magic: The Gathering game.
///
/// From the comprehensive rules:
/// Anything that happens in a game. See rule 700.1.
///
/// See also https://mtg.fandom.com/wiki/Event
///
/// We keep a smaller list here, that are used to parse the cards.
/// All events here are the ones encountered in triggered abilities / replacement effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Event {
    CreateTokens(CreateTokensEvent),
    CreatureAction(CreatureActionEvent),
    EntersTheBattlefield(EntersTheBattlefieldEvent),
    LifeGained(LifeGainedEvent),
    PlayerAction(PlayerActionEvent),
    PutCounterOnPermanent(PutCounterOnPermanentEvent),
}

impl crate::ability_tree::AbilityTreeNode for Event {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Event.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreateTokens(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CreatureAction(child) => children.push(child as &dyn AbilityTreeNode),
            Self::EntersTheBattlefield(child) => children.push(child as &dyn AbilityTreeNode),
            Self::LifeGained(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlayerAction(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PutCounterOnPermanent(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event:")?;
        out.push_final_branch()?;
        match self {
            Self::CreateTokens(event) => event.display(out)?,
            Self::CreatureAction(event) => event.display(out)?,
            Self::EntersTheBattlefield(event) => event.display(out)?,
            Self::LifeGained(event) => event.display(out)?,
            Self::PlayerAction(event) => event.display(out)?,
            Self::PutCounterOnPermanent(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Event {
    fn dummy_init() -> Self {
        Self::CreateTokens(crate::utils::dummy())
    }
}
