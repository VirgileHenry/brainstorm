mod event_occured;
mod object_is_of_kind;
mod this_is_your_turn;

pub use event_occured::ConditionEventOccured;
pub use object_is_of_kind::ConditionObjectMatchSpecifiers;
pub use this_is_your_turn::ConditionThisIsYourTurn;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A conditional clause allows ability to only trigger or happen when some other
/// conditions are met.
///
/// There are two kind of contional, an "if" that requires that a condition is met,
/// and an "unless" that requires that the condition has not been met.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Conditional {
    If(ConditionalIf),
    Unless(ConditionalUnless),
}

impl AbilityTreeNode for Conditional {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Conditional.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::If(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Unless(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "conditional:")?;
        out.push_final_branch()?;
        match self {
            Self::If(child) => child.display(out)?,
            Self::Unless(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Conditional {
    fn dummy_init() -> Self {
        Self::If(crate::utils::dummy())
    }
}

/// "If" variant of the [`Conditional`].
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ConditionalIf {
    pub condition: Condition,
}

impl AbilityTreeNode for ConditionalIf {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionalIf.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "if:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionalIf {
    fn dummy_init() -> Self {
        Self {
            condition: crate::utils::dummy(),
        }
    }
}

/// "Unless" variant of the [`Conditional`].
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ConditionalUnless {
    pub condition: Condition,
}

impl AbilityTreeNode for ConditionalUnless {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionalUnless.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "unless:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionalUnless {
    fn dummy_init() -> Self {
        Self {
            condition: crate::utils::dummy(),
        }
    }
}

/// A condition regroups what can be used as conditions for conditinals.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Condition {
    EventOccured(ConditionEventOccured),
    ObjectMatchSpecifiers(ConditionObjectMatchSpecifiers),
    ThisIsYourTurn(ConditionThisIsYourTurn),
}

impl AbilityTreeNode for Condition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Condition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::EventOccured(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ObjectMatchSpecifiers(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ThisIsYourTurn(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "condition:")?;
        out.push_final_branch()?;
        match self {
            Self::EventOccured(child) => child.display(out)?,
            Self::ObjectMatchSpecifiers(child) => child.display(out)?,
            Self::ThisIsYourTurn(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Condition {
    fn dummy_init() -> Self {
        Self::ThisIsYourTurn(crate::utils::dummy())
    }
}
