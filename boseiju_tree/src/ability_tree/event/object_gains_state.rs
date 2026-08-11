mod creature_gains_state_event;
mod permanent_gains_state_event;
mod spell_gains_state_event;

pub use creature_gains_state_event::CreatureGainsStateEvent;
pub use permanent_gains_state_event::PermanentGainsStateEvent;
pub use spell_gains_state_event::SpellGainsStateEvent;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An event for when objects gains states.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectGainsStateEvent {
    CreatureGainsState(CreatureGainsStateEvent),
    PermanentGainsState(PermanentGainsStateEvent),
    SpellGainsState(SpellGainsStateEvent),
}

impl crate::Node for ObjectGainsStateEvent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ObjectGainsStateEvent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreatureGainsState(child) => children.push(child as &dyn Node),
            Self::PermanentGainsState(child) => children.push(child as &dyn Node),
            Self::SpellGainsState(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object gains state event:")?;
        out.push_final_branch()?;
        match self {
            Self::CreatureGainsState(event) => event.display(out)?,
            Self::PermanentGainsState(event) => event.display(out)?,
            Self::SpellGainsState(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object gains state event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectGainsStateEvent {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CreatureGainsState(child) => child.span(),
            Self::PermanentGainsState(child) => child.span(),
            Self::SpellGainsState(child) => child.span(),
        }
    }
}

impl Default for ObjectGainsStateEvent {
    fn default() -> Self {
        Self::CreatureGainsState(Default::default())
    }
}
