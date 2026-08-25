use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Deed for destroying an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destroy<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub object: crate::ability_tree::object::Permanent<Q>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for Destroy<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::Destroy)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "destroy:")?;
        out.push_final_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "destroy"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for Destroy<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for Destroy<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            object: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub type DestroyActive = Destroy<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type DestroyPassive = Destroy<crate::ability_tree::quantifier::PassiveQuantifier>;
