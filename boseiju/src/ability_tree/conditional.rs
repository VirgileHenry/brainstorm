mod event_occured;
mod number_of_resolution;
mod object_is_of_kind;
mod player_controls_permanent;
mod this_is_your_turn;

pub use event_occured::ConditionEventOccured;
pub use number_of_resolution::ConditionNumberOfResolutions;
pub use object_is_of_kind::ConditionCreatureMatchSpecifier;
pub use player_controls_permanent::PlayerControlsPermanent;
pub use this_is_your_turn::ConditionThisIsYourTurn;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A conditional clause allows ability to only trigger or happen when some other
/// conditions are met.
///
/// There are two kind of contional, an "if" that requires that a condition is met,
/// and an "unless" that requires that the condition has not been met.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn node_tag(&self) -> &'static str {
        "conditional"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::If(child) => child.node_span(),
            Self::Unless(child) => child.node_span(),
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalIf {
    pub condition: Condition,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "if condition"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionalIf {
    fn dummy_init() -> Self {
        Self {
            condition: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// "Unless" variant of the [`Conditional`].
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalUnless {
    pub condition: Condition,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "unless condition"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionalUnless {
    fn dummy_init() -> Self {
        Self {
            condition: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A condition regroups what can be used as conditions for conditinals.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    EventOccured(ConditionEventOccured),
    NumberOfResolutions(ConditionNumberOfResolutions),
    ObjectMatchSpecifiers(ConditionCreatureMatchSpecifier),
    PlayerControlsObject(PlayerControlsPermanent),
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
            Self::NumberOfResolutions(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ObjectMatchSpecifiers(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlayerControlsObject(child) => children.push(child as &dyn AbilityTreeNode),
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
            Self::NumberOfResolutions(child) => child.display(out)?,
            Self::ObjectMatchSpecifiers(child) => child.display(out)?,
            Self::PlayerControlsObject(child) => child.display(out)?,
            Self::ThisIsYourTurn(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "condition"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::EventOccured(child) => child.node_span(),
            Self::NumberOfResolutions(child) => child.node_span(),
            Self::ObjectMatchSpecifiers(child) => child.node_span(),
            Self::PlayerControlsObject(child) => child.node_span(),
            Self::ThisIsYourTurn(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Condition {
    fn dummy_init() -> Self {
        Self::ThisIsYourTurn(crate::utils::dummy())
    }
}
