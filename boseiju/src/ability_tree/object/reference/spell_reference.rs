use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A Spell reference.
///
/// This can only reference artifacts on the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellReference {
    pub count: crate::ability_tree::object::CountSpecifier,
    pub kind: crate::ability_tree::object::kind::SpellKind,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for SpellReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpellReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn AbilityTreeNode);
        children.push(&self.kind as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpellReference {
    fn dummy_init() -> Self {
        Self {
            count: crate::utils::dummy(),
            kind: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
