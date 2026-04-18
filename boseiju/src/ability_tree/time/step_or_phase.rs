use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Either a step or a phase, both discrete time elements.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOrPhase {
    Step(crate::ability_tree::terminals::Step),
    Phase(crate::ability_tree::terminals::Phase),
}

impl AbilityTreeNode for StepOrPhase {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StepOrPhase.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Step(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Phase(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "step or phase:")?;
        out.push_final_branch()?;
        match self {
            Self::Step(child) => child.display(out)?,
            Self::Phase(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "step or phase"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Step(child) => child.node_span(),
            Self::Phase(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StepOrPhase {
    fn dummy_init() -> Self {
        Self::Step(crate::utils::dummy())
    }
}
