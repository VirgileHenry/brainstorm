use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A literal number in an ability, such as "1", "two", "10"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyNumber {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for AnyNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::NumberAnyNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "any number")
    }

    fn node_tag(&self) -> &'static str {
        "any number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AnyNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for AnyNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for AnyNumber {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
