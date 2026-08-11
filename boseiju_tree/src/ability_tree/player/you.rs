use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct You {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for You {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::You
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "you")
    }

    fn node_tag(&self) -> &'static str {
        "you"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for You {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for You {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for You {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
