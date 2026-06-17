mod tapped_state;
mod target_state;
mod untapped_state;

pub use tapped_state::PermanentTappedState;
pub use target_state::PermanentTargetedState;
pub use untapped_state::PermanentUntappedState;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

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

impl AbilityTreeNode for PermanentState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Tapped(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Untapped(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Targeted(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Tapped(child) => child.node_span(),
            Self::Untapped(child) => child.node_span(),
            Self::Targeted(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentState {
    fn dummy_init() -> Self {
        Self::Tapped(crate::utils::dummy())
    }
}
