use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionStackObjectHasState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ConditionStackObjectHasState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.stack_obj as &dyn Node);
        children.push(&self.state as &dyn Node);
        children
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Boolean { value: self.has_state })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionStackObjectHasState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionStackObjectHasState {
    fn default() -> Self {
        Self {
            stack_obj: Default::default(),
            state: Default::default(),
            has_state: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
