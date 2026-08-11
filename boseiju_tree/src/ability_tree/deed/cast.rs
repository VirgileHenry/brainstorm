use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An deed for a spell being cast.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cast<F>
where
    F: super::form::DeedForm,
{
    pub caster: crate::ability_tree::player::PlayerReference<F::Quantifier>,
    pub spell: crate::ability_tree::object::Spell,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<F> crate::Node for Cast<F>
where
    F: super::form::DeedForm,
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
impl<F> boseiju_span::Spanned for Cast<F>
where
    F: super::form::DeedForm,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<F> Default for Cast<F>
where
    F: super::form::DeedForm,
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
