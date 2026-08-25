use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A Enchantment reference.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnchantmentReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub quantifier: Q,
    pub enchantment: crate::ability_tree::object::specified_object::SpecifiedEnchantment,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> Node for EnchantmentReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EnchantmentReference
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.quantifier as &dyn Node);
        children.push(&self.enchantment as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enchantment reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.quantifier.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "enchantment:")?;
        out.push_final_branch()?;
        self.enchantment.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enchantment reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for EnchantmentReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for EnchantmentReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            quantifier: Default::default(),
            enchantment: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
