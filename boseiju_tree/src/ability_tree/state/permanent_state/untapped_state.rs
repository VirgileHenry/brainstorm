use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PermanentUntappedState {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PermanentUntappedState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PermanentUntappedState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "untapped state")
    }

    fn node_tag(&self) -> &'static str {
        "untapped state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentUntappedState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PermanentUntappedState {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
