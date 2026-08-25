use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Deed for a spell being cast.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cast<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub caster: crate::ability_tree::player::PlayerReference<Q>,
    pub spell: crate::ability_tree::object::PassiveSpell,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for Cast<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::Cast)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.caster as &dyn Node);
        children.push(&self.spell as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cast:")?;
        out.push_inter_branch()?;
        write!(out, "caster:")?;
        out.push_final_branch()?;
        self.caster.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "spell:")?;
        out.push_final_branch()?;
        self.spell.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cast"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for Cast<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for Cast<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn default() -> Self {
        Self {
            caster: Default::default(),
            spell: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub type CastActive = Cast<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type CastPassive = Cast<crate::ability_tree::quantifier::PassiveQuantifier>;
