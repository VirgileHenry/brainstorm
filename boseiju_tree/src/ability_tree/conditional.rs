mod event_occured;
mod object_is_of_kind;
mod player_controls_permanent;
mod stack_object_has_state;
mod this_is_your_turn;

pub use event_occured::ConditionEventOccured;
pub use object_is_of_kind::ConditionCreatureMatchSpecifier;
pub use player_controls_permanent::ConditionPlayerControlsPermanent;
pub use stack_object_has_state::ConditionStackObjectHasState;
pub use this_is_your_turn::ConditionThisIsYourTurn;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

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

impl Node for Conditional {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Conditional.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::If(child) => children.push(child as &dyn Node),
            Self::Unless(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Conditional {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::If(child) => child.span(),
            Self::Unless(child) => child.span(),
        }
    }
}


impl Default for Conditional {
    fn default() -> Self {
        Self::If(Default::default())
    }
}

/// "If" variant of the [`Conditional`].
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalIf {
    pub condition: Condition,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ConditionalIf {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ConditionalIf.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionalIf {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}


impl Default for ConditionalIf {
    fn default() -> Self {
        Self {
            condition: Default::default(),
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
    pub span: boseiju_span::Span,
}

impl Node for ConditionalUnless {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ConditionalUnless.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionalUnless {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}


impl Default for ConditionalUnless {
    fn default() -> Self {
        Self {
            condition: Default::default(),
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
    ObjectMatchSpecifiers(ConditionCreatureMatchSpecifier),
    PlayerControlsObject(ConditionPlayerControlsPermanent),
    StackObjectHasState(ConditionStackObjectHasState),
    ThisIsYourTurn(ConditionThisIsYourTurn),
}

impl Node for Condition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Condition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::EventOccured(child) => children.push(child as &dyn Node),
            Self::ObjectMatchSpecifiers(child) => children.push(child as &dyn Node),
            Self::PlayerControlsObject(child) => children.push(child as &dyn Node),
            Self::StackObjectHasState(child) => children.push(child as &dyn Node),
            Self::ThisIsYourTurn(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "condition:")?;
        out.push_final_branch()?;
        match self {
            Self::EventOccured(child) => child.display(out)?,
            Self::ObjectMatchSpecifiers(child) => child.display(out)?,
            Self::PlayerControlsObject(child) => child.display(out)?,
            Self::StackObjectHasState(child) => child.display(out)?,
            Self::ThisIsYourTurn(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "condition"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Condition {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::EventOccured(child) => child.span(),
            Self::ObjectMatchSpecifiers(child) => child.span(),
            Self::PlayerControlsObject(child) => child.span(),
            Self::StackObjectHasState(child) => child.span(),
            Self::ThisIsYourTurn(child) => child.span(),
        }
    }
}


impl Default for Condition {
    fn default() -> Self {
        Self::ThisIsYourTurn(Default::default())
    }
}
