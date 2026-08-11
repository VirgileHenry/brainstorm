mod creature_performs_action;
mod object_gains_state;
mod permanent_performs_action;
mod player_performs_action;

pub use creature_performs_action::*;
pub use object_gains_state::*;
pub use permanent_performs_action::*;
pub use player_performs_action::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
    CreaturePerformsAction(CreaturePerformsActionEvent),
    ObjectGainsState(ObjectGainsStateEvent),
    PermanentPerformsAction(PermanentPerformsActionEvent),
    PlayerPerformsAction(PlayerPerformsActionEvent),
}

impl crate::Node for Event {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Event
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreaturePerformsAction(child) => children.push(child as &dyn Node),
            Self::ObjectGainsState(child) => children.push(child as &dyn Node),
            Self::PermanentPerformsAction(child) => children.push(child as &dyn Node),
            Self::PlayerPerformsAction(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event:")?;
        out.push_final_branch()?;
        match self {
            Self::CreaturePerformsAction(event) => event.display(out)?,
            Self::ObjectGainsState(event) => event.display(out)?,
            Self::PermanentPerformsAction(event) => event.display(out)?,
            Self::PlayerPerformsAction(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Event {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CreaturePerformsAction(child) => child.span(),
            Self::ObjectGainsState(child) => child.span(),
            Self::PermanentPerformsAction(child) => child.span(),
            Self::PlayerPerformsAction(child) => child.span(),
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::CreaturePerformsAction(Default::default())
    }
}
