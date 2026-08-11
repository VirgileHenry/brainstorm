mod incoming_instant;
mod recurrent_instant;
mod step_or_phase;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

pub use incoming_instant::*;
pub use recurrent_instant::*;
pub use step_or_phase::*;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instant {
    Incoming(IncomingInstant),
    Reccurent(RecurrentInstant),
}

impl Node for Instant {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Instant
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Incoming(child) => children.push(child as &dyn Node),
            Self::Reccurent(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "instant:")?;
        out.push_final_branch()?;
        match self {
            Self::Incoming(child) => child.display(out)?,
            Self::Reccurent(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "instant"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Instant {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Incoming(child) => child.span(),
            Self::Reccurent(child) => child.span(),
        }
    }
}

impl Default for Instant {
    fn default() -> Self {
        Self::Reccurent(Default::default())
    }
}
