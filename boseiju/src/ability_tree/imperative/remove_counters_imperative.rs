use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_COUNTER_AMOUNT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Imperative to put counters on objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct RemoveCountersImperative {
    pub object: crate::ability_tree::object::ObjectReference,
    pub counters: arrayvec::ArrayVec<RemovableCounterOnPermanent, MAX_COUNTER_AMOUNT>,
}

impl AbilityTreeNode for RemoveCountersImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PutCountersImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        for counter in self.counters.iter() {
            children.push(counter as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "remove counters:")?;
        out.push_inter_branch()?;
        write!(out, "on object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "counters:")?;
        for (i, counter) in self.counters.iter().enumerate() {
            if i == self.counters.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            counter.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for RemoveCountersImperative {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            counters: crate::utils::dummy(),
        }
    }
}

/// An amount and a kind of counters to be put on a permanent.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct RemovableCounterOnPermanent {
    pub amount: crate::ability_tree::number::Number,
    pub counter: RemovableCounterKind,
}

impl crate::ability_tree::AbilityTreeNode for RemovableCounterOnPermanent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::RemovableCounterOnPermanent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn AbilityTreeNode);
        children.push(&self.counter as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "counter on permanent:")?;
        out.push_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "of counter:")?;
        out.push_final_branch()?;
        self.counter.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

/// Kind of counter that is put on a permanent.
///
/// It's either a given kind of counter, e.g. "put a shield counter" or
/// a previously mentionned kind of counter, e.g. "that many counters on...".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum RemovableCounterKind {
    AnyCounter,
    NewCounter(crate::ability_tree::terminals::Counter),
}

impl AbilityTreeNode for RemovableCounterKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::RemovableCounterKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AnyCounter => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::PreviouslyMentionnedCounter.id(),
            ) as &dyn AbilityTreeNode),
            Self::NewCounter(counter) => children.push(counter as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "counter kind:")?;
        out.push_final_branch()?;
        match self {
            Self::AnyCounter => write!(out, "any kind of counter")?,
            Self::NewCounter(counter) => counter.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for RemovableCounterKind {
    fn dummy_init() -> Self {
        Self::AnyCounter
    }
}
