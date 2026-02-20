use super::*;
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PutCounterOnPermanentEvent {
    pub source: source::EventSource,
    pub quantity: crate::ability_tree::number::Number,
    pub on_permanent: crate::ability_tree::object::ObjectReference,
    pub counter_kind: Option<crate::ability_tree::terminals::Counter>,
}

impl crate::ability_tree::AbilityTreeNode for PutCounterOnPermanentEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PutCounterOnPermanentEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source as &dyn AbilityTreeNode);
        children.push(&self.quantity as &dyn AbilityTreeNode);
        children.push(&self.on_permanent as &dyn AbilityTreeNode);
        match self.counter_kind.as_ref() {
            Some(amount) => children.push(amount as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters on permanents")?;
        out.push_inter_branch()?;
        write!(out, "put counters source:")?;
        out.push_final_branch()?;
        self.source.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.quantity.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        match self.counter_kind.as_ref() {
            Some(counter) => write!(out, "counter kind: {counter}")?,
            None => write!(out, "counter kind: any")?,
        }
        out.next_final_branch()?;
        write!(out, "on permanent:")?;
        out.push_final_branch()?;
        self.on_permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();

        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PutCounterOnPermanentEvent {
    fn dummy_init() -> Self {
        Self {
            source: crate::utils::dummy(),
            quantity: crate::utils::dummy(),
            on_permanent: crate::utils::dummy(),
            counter_kind: crate::utils::dummy(),
        }
    }
}
