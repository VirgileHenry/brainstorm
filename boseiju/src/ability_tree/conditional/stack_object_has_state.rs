use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A condition that is met when a given object matches given object specifiers.
///
/// For example, "if it is a zombie card".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionStackObjectHasState {
    pub stack_obj: crate::ability_tree::object::Spell, /* Fixme: stack object */
    pub state: crate::ability_tree::state::StackObjectState,
    pub has_state: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ConditionStackObjectHasState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionStackObjectHasState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.stack_obj as &dyn AbilityTreeNode);
        children.push(&self.state as &dyn AbilityTreeNode);
        children
    }

    fn data(&self) -> Option<crate::ability_tree::AbTreeNodeData> {
        Some(crate::ability_tree::AbTreeNodeData::Boolean { value: self.has_state })
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "stack object has state:")?;
        out.push_inter_branch()?;
        self.stack_obj.display(out)?;
        out.next_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "stack object has state"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionStackObjectHasState {
    fn dummy_init() -> Self {
        Self {
            stack_obj: crate::utils::dummy(),
            state: crate::utils::dummy(),
            has_state: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
