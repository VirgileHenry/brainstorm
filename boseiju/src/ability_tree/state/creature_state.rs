use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// States that only creatures can have.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureState {
    /// Attacking creature state.
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Blocking creature state.
    Blocking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Equipped creature state.
    Equipped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for CreatureState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureStateIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let child_id = crate::ability_tree::NodeKind::CreatureState(self.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature state:")?;
        out.push_final_branch()?;
        match self {
            Self::Attacking { .. } => write!(out, "attacking")?,
            Self::Blocking { .. } => write!(out, "blocking")?,
            Self::Equipped { .. } => write!(out, "blocking")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature state"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacking { span } => *span,
            Self::Blocking { span } => *span,
            Self::Equipped { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureState {
    fn dummy_init() -> Self {
        Self::Attacking {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
