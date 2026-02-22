use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CounterOnPermanentReplacement {
    pub source_ref: super::EventSourceReference,
    pub put_counters: crate::ability_tree::imperative::PutCountersImperative,
}

impl AbilityTreeNode for CounterOnPermanentReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CounterOnPermanentReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source_ref as &dyn AbilityTreeNode);
        children.push(&self.put_counters as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters replacement:")?;
        out.push_inter_branch()?;
        write!(out, "effect source:")?;
        out.push_final_branch()?;
        self.source_ref.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "put counters:")?;
        out.push_final_branch()?;
        self.put_counters.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CounterOnPermanentReplacement {
    fn dummy_init() -> Self {
        Self {
            source_ref: crate::utils::dummy(),
            put_counters: crate::utils::dummy(),
        }
    }
}
