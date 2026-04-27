use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// The self referencing struct is a special kind of object specifier
/// that references the objects that carries the ability.
///
/// For instance, in the ability text "when this creature enters, ...",
/// "this creature" is a self referencing keyword.
///
/// Prior to the Foundation (FDN) set, self referencing was done by mentionning the
/// name of the card, either the full name or without the epiphet.
///
/// Since FDN, self referencing can be done through "this card / creature / etc".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelfReferencing {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for SelfReferencing {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::SelfReferencing).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "self referencing")
    }

    fn node_tag(&self) -> &'static str {
        "self referencing"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SelfReferencing {
    fn dummy_init() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
