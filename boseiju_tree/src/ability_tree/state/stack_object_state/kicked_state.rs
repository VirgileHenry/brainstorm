use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The "kicked" state for stack objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KickedState {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for KickedState {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::StackObjectStateKicked
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "kicked")
    }

    fn node_tag(&self) -> &'static str {
        "kicked state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for KickedState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for KickedState {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for KickedState {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
