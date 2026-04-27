use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Marker struct for the special object specifier "another",
/// which means "any that is not myself".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnotherObjectSpecifier {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for AnotherObjectSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::AnotherObjectSpecifier).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "another object specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AnotherObjectSpecifier {
    fn dummy_init() -> Self {
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
