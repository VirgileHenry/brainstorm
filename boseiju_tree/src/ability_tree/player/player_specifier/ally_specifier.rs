use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllySpecifier {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for AllySpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::AllySpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "ally player specifier")
    }

    fn node_tag(&self) -> &'static str {
        "ally player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AllySpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for AllySpecifier {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for AllySpecifier {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
