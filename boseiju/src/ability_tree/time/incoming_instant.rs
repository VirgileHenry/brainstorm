use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An incoming instant is an instant that is defined from the moment it is resolved.
/// For instance, "the beginning of your next turn" is an incoming instant.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingInstant {
    /// A next specific step, for instance, "your next end step"
    NextStepOrPhase(IncomingNextStepOrPhase),
    /// A step or phase in player the next turn, "end of your next turn"
    StepInNextTurn(IncomingStepInNextTurn),
}

impl AbilityTreeNode for IncomingInstant {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::IncomingInstant.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NextStepOrPhase(child) => children.push(child as &dyn AbilityTreeNode),
            Self::StepInNextTurn(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::NextStepOrPhase(child) => child.node_span(),
            Self::StepInNextTurn(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for IncomingInstant {
    fn dummy_init() -> Self {
        Self::NextStepOrPhase(crate::utils::dummy())
    }
}

/// An instant represented by the next step or next phase.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingNextStepOrPhase {
    pub step_or_phase: crate::ability_tree::time::StepOrPhase,
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for IncomingNextStepOrPhase {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::IncomingNextStepOrPhase.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for IncomingNextStepOrPhase {
    fn dummy_init() -> Self {
        Self {
            step_or_phase: crate::utils::dummy(),
            owner: crate::utils::dummy(),
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
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for IncomingStepInNextTurn {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::IncomingStepInNextTurn.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for IncomingStepInNextTurn {
    fn dummy_init() -> Self {
        Self {
            step_or_phase: crate::utils::dummy(),
            owner: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
