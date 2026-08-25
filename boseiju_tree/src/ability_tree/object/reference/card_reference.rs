use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A Card reference.
///
/// This can only reference artifacts on the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub quantifier: Q,
    pub card: crate::ability_tree::object::specified_object::SpecifiedCard,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> Node for CardReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CardReference
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.quantifier as &dyn Node);
        children.push(&self.card as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.quantifier.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "card:")?;
        out.push_final_branch()?;
        self.card.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for CardReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for CardReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            quantifier: Default::default(),
            card: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
