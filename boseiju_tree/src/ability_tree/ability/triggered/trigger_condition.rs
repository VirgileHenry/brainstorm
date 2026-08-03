use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Condition for a trigger ability.
///
/// This is always an event, and can optionnaly have conditions for the event to happen.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerCondition {
    pub kind: TriggerConditionKind,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for TriggerCondition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::TriggerCondition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn Node),
            None => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "trigger condition:")?;
        out.push_inter_branch()?;
        self.kind.display(out)?;
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "trigger condition"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TriggerCondition {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for TriggerCondition {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            condition: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// Kind of trigger condition
///
/// This is always an event, and can optionnaly have conditions for the event to happen.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerConditionKind {
    Event(crate::ability_tree::event::Event),
    /// Triggered ability that triggers at a given instant.
    /// The instant has to be a recurrent instant I think ?
    /// Maybe some cards will prove me wrong here
    AtInstant(crate::ability_tree::time::RecurrentInstant),
}

impl Node for TriggerConditionKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::TriggerConditionKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Event(child) => children.push(child as &dyn Node),
            Self::AtInstant(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "trigger condition kind:")?;
        out.push_final_branch()?;
        match self {
            Self::Event(child) => child.display(out)?,
            Self::AtInstant(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "trigger condition"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TriggerConditionKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Event(child) => child.span(),
            Self::AtInstant(child) => child.span(),
        }
    }
}

impl Default for TriggerConditionKind {
    fn default() -> Self {
        Self::Event(Default::default())
    }
}
