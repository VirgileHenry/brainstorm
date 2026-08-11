mod tapped_state;
mod target_state;
mod untapped_state;

pub use tapped_state::PermanentTappedState;
pub use target_state::PermanentTargetedState;
pub use untapped_state::PermanentUntappedState;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// States that only creatures can have.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentState {
    /// Tapped permanent state
    Tapped(PermanentTappedState),
    /// Untapped permanent state
    Untapped(PermanentUntappedState),
    /// Being the target of a stack object permanent state.
    Targeted(PermanentTargetedState),
}

impl Node for PermanentState {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PermanentState
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Tapped(child) => children.push(child as &dyn Node),
            Self::Untapped(child) => children.push(child as &dyn Node),
            Self::Targeted(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent state:")?;
        out.push_final_branch()?;
        match self {
            Self::Tapped(child) => child.display(out)?,
            Self::Untapped(child) => child.display(out)?,
            Self::Targeted(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentState {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Tapped(child) => child.span(),
            Self::Untapped(child) => child.span(),
            Self::Targeted(child) => child.span(),
        }
    }
}

impl Default for PermanentState {
    fn default() -> Self {
        Self::Tapped(Default::default())
    }
}
