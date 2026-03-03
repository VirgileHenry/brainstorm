use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// The self referencing struct is a special kind of object specifier
/// that references the objects that carries the ability.
///
/// For instance, in the ability text "when this creature enters, ...",
/// "this creature" is a self referencing keyword.
///
/// Prior to the Foundation (FDN) set, self referencing was done by mentionning the
/// name of the card, either the full name or without the epiphet.
///
/// Since FDN, self referencing can be done through "this <object kind>".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelfReferencingObject {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for SelfReferencingObject {
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
        write!(out, "{self}")
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl std::fmt::Display for SelfReferencingObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "self referencing (~)")
    }
}

impl IntoToken for SelfReferencingObject {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "this creature" => Some(SelfReferencingObject {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this spell" => Some(SelfReferencingObject {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this card" => Some(SelfReferencingObject {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this token" => Some(SelfReferencingObject {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "~" => Some(SelfReferencingObject {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SelfReferencingObject {
    fn dummy_init() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
