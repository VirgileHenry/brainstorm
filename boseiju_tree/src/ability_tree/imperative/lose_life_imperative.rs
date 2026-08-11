use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Imperative to draw cards or make a player draw cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoseLifeImperative {
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for LoseLifeImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::LoseLifeImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "lose life:")?;
        out.push_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "lose life imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LoseLifeImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for LoseLifeImperative {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
