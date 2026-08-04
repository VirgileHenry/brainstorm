use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpponentSpecifier {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for OpponentSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::OpponentSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "opponent player specifier")
    }

    fn node_tag(&self) -> &'static str {
        "opponent player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OpponentSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for OpponentSpecifier {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for OpponentSpecifier {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
