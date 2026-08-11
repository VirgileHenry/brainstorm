mod countered_state;
mod kicked_state;

pub use countered_state::CounteredState;
pub use kicked_state::KickedState;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// States that only creatures can have.
///
/// Fixme: We should have separate structs ? like multi kicker and all
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObjectState {
    /// Countered spell state.
    Countered(CounteredState),
    /// Kicked spell state.
    Kicked(KickedState),
}

impl Node for StackObjectState {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::StackObjectState
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Countered(child) => children.push(child as &dyn Node),
            Self::Kicked(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "stack object state:")?;
        out.push_final_branch()?;
        match self {
            Self::Countered(child) => child.display(out)?,
            Self::Kicked(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "stack object state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StackObjectState {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Countered(child) => child.span(),
            Self::Kicked(child) => child.span(),
        }
    }
}

impl Default for StackObjectState {
    fn default() -> Self {
        Self::Countered(Default::default())
    }
}
