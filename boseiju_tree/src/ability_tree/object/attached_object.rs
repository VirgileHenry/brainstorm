use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Object reference for the object "attached" to.
///
/// This only has meaning when the ability is on a card that can
/// be attached to objects, and this references those objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttachedObject {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for AttachedObject {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::AttachedObject
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object attached to")
    }

    fn node_tag(&self) -> &'static str {
        "object attached to"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AttachedObject {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}


impl Default for AttachedObject {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
