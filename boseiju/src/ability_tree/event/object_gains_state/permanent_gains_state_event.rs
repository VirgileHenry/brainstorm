use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event for when a permanent gains a permanent state.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentGainsStateEvent {
    pub permanent: crate::ability_tree::object::Permanent,
    pub state: crate::ability_tree::state::PermanentState,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PermanentGainsStateEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentGainsStateEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn AbilityTreeNode);
        children.push(&self.state as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent gains state event:")?;
        out.push_inter_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "gains state:")?;
        out.push_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent gains state event"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentGainsStateEvent {
    fn dummy_init() -> Self {
        Self {
            permanent: crate::utils::dummy(),
            state: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
