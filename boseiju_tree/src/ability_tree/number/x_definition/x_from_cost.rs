use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An imperative for tapping an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XFromCost {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for XFromCost {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::XFromCost
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x from cost")?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x from cost"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for XFromCost {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for XFromCost {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
