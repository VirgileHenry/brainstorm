use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Imperative to draw cards or make a player draw cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardImperative {
    pub card: crate::ability_tree::object::Card,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for DiscardImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DestroyImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "discard:")?;
        out.push_final_branch()?;
        write!(out, "cards:")?;
        out.push_final_branch()?;
        self.card.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "discard imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DiscardImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for DiscardImperative {
    fn default() -> Self {
        Self {
            card: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
