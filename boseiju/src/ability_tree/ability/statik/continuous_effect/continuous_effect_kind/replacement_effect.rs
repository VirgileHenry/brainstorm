use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A replacement effect replaces entirely one event with another.
///
/// https://mtg.fandom.com/wiki/Replacement_effect
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ContinuousEffectReplacementEvent {
    pub replaced_event: crate::ability_tree::event::Event,
    pub replaced_by: crate::ability_tree::event::replacement::EventReplacement,
}

impl AbilityTreeNode for ContinuousEffectReplacementEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectReplacementEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.replaced_event as &dyn AbilityTreeNode);
        children.push(&self.replaced_by as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "replacement effect:")?;
        out.push_inter_branch()?;
        write!(out, "replaced event:")?;
        out.push_final_branch()?;
        self.replaced_event.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "replaced by:")?;
        out.push_final_branch()?;
        self.replaced_by.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffectReplacementEvent {
    fn dummy_init() -> Self {
        Self {
            replaced_event: crate::utils::dummy(),
            replaced_by: crate::utils::dummy(),
        }
    }
}
