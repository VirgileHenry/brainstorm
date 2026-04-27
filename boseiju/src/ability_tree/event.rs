mod creature_performs_action;
mod object_gains_state;
mod permanent_performs_action;

pub use creature_performs_action::*;
pub use object_gains_state::*;
pub use permanent_performs_action::*;

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
    CreaturePerformsAction(CreaturePerformsActionEvent),
    ObjectGainsState(ObjectGainsStateEvent),
    PermanentPerformsAction(PermanentPerformsActionEvent),
}

impl crate::ability_tree::AbilityTreeNode for Event {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Event.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreaturePerformsAction(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ObjectGainsState(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PermanentPerformsAction(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event:")?;
        out.push_final_branch()?;
        match self {
            Self::CreaturePerformsAction(event) => event.display(out)?,
            Self::ObjectGainsState(event) => event.display(out)?,
            Self::PermanentPerformsAction(event) => event.display(out)?,
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
            Self::CreaturePerformsAction(child) => child.node_span(),
            Self::ObjectGainsState(child) => child.node_span(),
            Self::PermanentPerformsAction(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Event {
    fn dummy_init() -> Self {
        Self::CreaturePerformsAction(crate::utils::dummy())
    }
}
