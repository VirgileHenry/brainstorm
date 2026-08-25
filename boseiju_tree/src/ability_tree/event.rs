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
pub struct Event {
    pub deed: crate::ability_tree::deed::PassiveFormDeed,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for Event {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Event
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.deed as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event:")?;
        out.push_final_branch()?;
        write!(out, "deed:")?;
        out.push_final_branch()?;
        self.deed.display(out)?;
        out.pop_branch();
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
        self.span
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            deed: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
