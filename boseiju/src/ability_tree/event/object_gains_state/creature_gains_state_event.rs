use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event for when a creature gains a creature state.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureGainsStateEvent {
    pub creature: crate::ability_tree::object::CreatureReference,
    pub state: crate::ability_tree::state::CreatureState,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CreatureGainsStateEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureGainsStateEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn AbilityTreeNode);
        children.push(&self.state as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature gains state event:")?;
        out.push_inter_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
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
        "creature gains state event"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureGainsStateEvent {
    fn dummy_init() -> Self {
        Self {
            creature: crate::utils::dummy(),
            state: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
