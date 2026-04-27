mod creature_gains_state_event;
mod permanent_gains_state_event;
mod spell_gains_state_event;

pub use creature_gains_state_event::CreatureGainsStateEvent;
pub use permanent_gains_state_event::PermanentGainsStateEvent;
pub use spell_gains_state_event::SpellGainsStateEvent;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event for when objects gains states.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectGainsStateEvent {
    CreatureGainsState(CreatureGainsStateEvent),
    PermanentGainsState(PermanentGainsStateEvent),
    SpellGainsState(SpellGainsStateEvent),
}

impl crate::ability_tree::AbilityTreeNode for ObjectGainsStateEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectGainsStateEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreatureGainsState(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PermanentGainsState(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SpellGainsState(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CreatureGainsState(child) => child.node_span(),
            Self::PermanentGainsState(child) => child.node_span(),
            Self::SpellGainsState(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectGainsStateEvent {
    fn dummy_init() -> Self {
        Self::CreatureGainsState(crate::utils::dummy())
    }
}
