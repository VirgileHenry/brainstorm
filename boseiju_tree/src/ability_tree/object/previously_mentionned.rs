use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
pub struct PreviouslyMentionned {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PreviouslyMentionned {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PreviouslyMentionned
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "previously mentionned")
    }

    fn node_tag(&self) -> &'static str {
        "previously mentionned"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PreviouslyMentionned {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PreviouslyMentionned {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
