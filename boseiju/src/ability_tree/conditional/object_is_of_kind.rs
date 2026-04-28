use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A condition that is met when a given object matches given object specifiers.
///
/// For example, "if it is a zombie card".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionCreatureMatchSpecifier {
    pub creature: crate::ability_tree::object::Creature,
    pub specifier: crate::ability_tree::object::specified_object::CreatureSpecifier,
    pub shall_match: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ConditionCreatureMatchSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionCreatureMatchSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn AbilityTreeNode);
        children.push(&self.specifier as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object matches specifiers:")?;
        out.push_inter_branch()?;
        self.creature.display(out)?;
        out.next_final_branch()?;
        self.specifier.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object is of kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionCreatureMatchSpecifier {
    fn dummy_init() -> Self {
        Self {
            creature: crate::utils::dummy(),
            specifier: crate::utils::dummy(),
            shall_match: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
