use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PermanentTappedState {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PermanentTappedState {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PermanentTappedState
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "tapped state")
    }

    fn node_tag(&self) -> &'static str {
        "tapped state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentTappedState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PermanentTappedState {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
