use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// States that only creatures can have.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureState {
    /// Attacking creature state.
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Blocking creature state.
    Blocking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Equipped creature state.
    Equipped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Node for CreatureState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureStateIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let child_id = crate::NodeKind::CreatureState(self.clone()).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureState {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Attacking { span } => *span,
            Self::Blocking { span } => *span,
            Self::Equipped { span } => *span,
        }
    }
}

impl Default for CreatureState {
    fn default() -> Self {
        Self::Attacking {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
