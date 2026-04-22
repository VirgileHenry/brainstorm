pub mod replacement;
pub mod source;

mod create_token_event;
mod creature_action_event;
mod enters_the_battlefield_event;
mod life_gained_event;
mod object_becomes_state;
mod player_action_event;
mod put_counter_on_permanent_event;

pub use create_token_event::*;
pub use creature_action_event::*;
pub use enters_the_battlefield_event::*;
pub use life_gained_event::*;
pub use object_becomes_state::*;
pub use player_action_event::*;
pub use put_counter_on_permanent_event::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event is anything that happens in a Magic: The Gathering game.
///
/// From the comprehensive rules:
/// Anything that happens in a game. See rule 700.1.
///
/// See also <https://mtg.fandom.com/wiki/Event>
///
/// We keep a smaller list here, that are used to parse the cards.
/// All events here are the ones encountered in triggered abilities / replacement effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    CreateTokens(CreateTokensEvent),
    CreatureAction(CreatureActionEvent),
    EntersTheBattlefield(EntersTheBattlefieldEvent),
    LifeGained(LifeGainedEvent),
    ObjectBecomesState(ObjectBecomesStateEvent),
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
            Self::ObjectBecomesState(child) => children.push(child as &dyn AbilityTreeNode),
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
            Self::ObjectBecomesState(event) => event.display(out)?,
            Self::PlayerAction(event) => event.display(out)?,
            Self::PutCounterOnPermanent(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "event"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CreateTokens(child) => child.node_span(),
            Self::CreatureAction(child) => child.node_span(),
            Self::EntersTheBattlefield(child) => child.node_span(),
            Self::LifeGained(child) => child.node_span(),
            Self::ObjectBecomesState(child) => child.node_span(),
            Self::PlayerAction(child) => child.node_span(),
            Self::PutCounterOnPermanent(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Event {
    fn dummy_init() -> Self {
        Self::CreateTokens(crate::utils::dummy())
    }
}
