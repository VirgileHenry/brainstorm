use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A Spell reference.
///
/// This can only reference artifacts on the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellReference {
    pub count: crate::ability_tree::quantifier::ActiveQuantifier,
    pub spell: crate::ability_tree::object::specified_object::SpecifiedSpell,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for SpellReference {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpellReference
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn Node);
        children.push(&self.spell as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "spell:")?;
        out.push_final_branch()?;
        self.spell.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpellReference {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpellReference {
    fn default() -> Self {
        Self {
            count: Default::default(),
            spell: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
