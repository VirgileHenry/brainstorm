use idris::Idris;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_COUNTER_ON_PERMANENT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CounterOnPermanentReplacement {
    source_ref: super::EventSourceReference,
    /* Fixme: replace with put counters imperative here */
    counters: arrayvec::ArrayVec<CounterOnPermanent, MAX_COUNTER_ON_PERMANENT>,
}

impl AbilityTreeNode for CounterOnPermanentReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CounterOnPermanentReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source_ref as &dyn AbilityTreeNode);
        for counter in self.counters.iter() {
            children.push(counter as &dyn AbilityTreeNode);
        }
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
        write!(out, "counters:")?;
        for (i, token) in self.counters.iter().enumerate() {
            if i == self.counters.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            token.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CounterOnPermanent {
    pub amount: crate::ability_tree::number::Number,
    pub counter: ReplacedCounterKind,
    pub on_permanent: crate::ability_tree::object::ObjectReference,
}

impl crate::ability_tree::AbilityTreeNode for CounterOnPermanent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CounterOnPermanent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn AbilityTreeNode);
        children.push(&self.counter as &dyn AbilityTreeNode);
        children.push(&self.on_permanent as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters on permanent:")?;
        out.push_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        write!(out, "number:")?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "of counter:")?;
        out.push_final_branch()?;
        self.counter.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "on permanent:")?;
        out.push_final_branch()?;
        self.on_permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ReplacedCounterKind {
    PreviouslyMentionnedCounter,
}

impl AbilityTreeNode for ReplacedCounterKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ReplacedCounterKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::PreviouslyMentionnedCounter => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::PreviouslyMentionnedCounter.id(),
            ) as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "counter kind:")?;
        out.push_final_branch()?;
        match self {
            Self::PreviouslyMentionnedCounter => write!(out, "previously mentionned counter")?,
        }
        out.pop_branch();
        Ok(())
    }
}
