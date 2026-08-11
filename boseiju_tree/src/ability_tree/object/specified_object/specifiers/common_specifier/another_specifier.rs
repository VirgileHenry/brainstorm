use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Marker struct for the special object specifier "another",
/// which means "any that is not myself".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnotherObjectSpecifier {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for AnotherObjectSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        use crate::node_kind::TerminalNodeKind;
        crate::NodeKind::Terminal(TerminalNodeKind::AnotherObjectSpecifier)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "another object specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AnotherObjectSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for AnotherObjectSpecifier {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl std::fmt::Display for AnotherObjectSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "others")
    }
}
