use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// States that only creatures can have.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObjectState {
    /// Countered spell state.
    Countered {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Kicked spell state.
    Kicked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl StackObjectState {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Countered {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Kicked {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }
}

impl Node for StackObjectState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::StackObjectStateIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let child_id = crate::NodeKind::StackObjectState(self.clone()).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "stack object state:")?;
        out.push_final_branch()?;
        match self {
            Self::Countered { .. } => write!(out, "countered")?,
            Self::Kicked { .. } => write!(out, "kicked")?,
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
            Self::Countered { span } => *span,
            Self::Kicked { span } => *span,
        }
    }
}

impl Default for StackObjectState {
    fn default() -> Self {
        Self::Countered {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
