use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConditionObjectMatchSpecifiers {
    pub object: crate::ability_tree::object::ObjectReference,
    pub specifiers: crate::ability_tree::object::ObjectSpecifiers,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ConditionObjectMatchSpecifiers {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionObjectMatchSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        children.push(&self.specifiers as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object matches specifiers:")?;
        out.push_inter_branch()?;
        self.object.display(out)?;
        out.next_final_branch()?;
        self.specifiers.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "condition: object is of kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionObjectMatchSpecifiers {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            specifiers: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
