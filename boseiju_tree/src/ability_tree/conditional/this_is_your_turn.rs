use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// If condition for event that only applies during your turn.
///
/// This condition will mostly appear silently with sentences like "when X during your turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionThisIsYourTurn {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionThisIsYourTurn {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ThisIsYourTurn.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "this is your turn")
    }

    fn node_tag(&self) -> &'static str {
        "this is your turn"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionThisIsYourTurn {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionThisIsYourTurn {
    fn default() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
