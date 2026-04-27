use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// States that only creatures can have.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentState {
    /// Tapped permanent property
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Untapped permanent property
    Untapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for PermanentState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentStateIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let child_id = crate::ability_tree::NodeKind::PermanentState(self.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent state:")?;
        out.push_final_branch()?;
        match self {
            Self::Tapped { .. } => write!(out, "tapped")?,
            Self::Untapped { .. } => write!(out, "untapped")?,
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
            Self::Tapped { span } => *span,
            Self::Untapped { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentState {
    fn dummy_init() -> Self {
        Self::Tapped {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
