use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An incoming instant is an instant that is defined from the moment it is resolved.
/// For instance, "the beginning of your next turn" is an incoming instant.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingInstant {
    /// A next specific step, for instance, "your next end step"
    NextStepOrPhase(IncomingNextStepOrPhase),
    /// A step or phase in player the next turn, "end of your next turn"
    StepInNextTurn(IncomingStepInNextTurn),
    /* Next time <event> */
}

impl Node for IncomingInstant {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::IncomingInstant.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NextStepOrPhase(child) => children.push(child as &dyn Node),
            Self::StepInNextTurn(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "incoming instant:")?;
        out.push_final_branch()?;
        match self {
            Self::NextStepOrPhase(child) => child.display(out)?,
            Self::StepInNextTurn(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "recurrent instant"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for IncomingInstant {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::NextStepOrPhase(child) => child.span(),
            Self::StepInNextTurn(child) => child.span(),
        }
    }
}

impl Default for IncomingInstant {
    fn default() -> Self {
        Self::NextStepOrPhase(Default::default())
    }
}

/// An instant represented by the next step or next phase.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingNextStepOrPhase {
    pub step_or_phase: crate::ability_tree::time::StepOrPhase,
    pub owner: crate::ability_tree::player::PlayerReference,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for IncomingNextStepOrPhase {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::IncomingNextStepOrPhase.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "incoming step or phase:")?;
        out.push_final_branch()?;
        self.step_or_phase.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "incoming step or phase instant"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for IncomingNextStepOrPhase {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for IncomingNextStepOrPhase {
    fn default() -> Self {
        Self {
            step_or_phase: Default::default(),
            owner: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// An instant represented by the next step or next phase.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingStepInNextTurn {
    pub step_or_phase: crate::ability_tree::time::StepOrPhase,
    pub owner: crate::ability_tree::player::PlayerReference,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for IncomingStepInNextTurn {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::IncomingStepInNextTurn.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "incoming step or phase:")?;
        out.push_final_branch()?;
        self.step_or_phase.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "incoming step or phase instant"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for IncomingStepInNextTurn {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for IncomingStepInNextTurn {
    fn default() -> Self {
        Self {
            step_or_phase: Default::default(),
            owner: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
