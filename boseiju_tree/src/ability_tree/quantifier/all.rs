use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct All {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for All {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::You.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "all")
    }

    fn node_tag(&self) -> &'static str {
        "all"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for All {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for All {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "All"
    }
}

impl Default for All {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
