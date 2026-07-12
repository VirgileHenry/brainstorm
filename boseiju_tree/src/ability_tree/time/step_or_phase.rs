use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Either a step or a phase, both discrete time elements.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOrPhase {
    Step(boseiju_lexer::terminal::Step),
    Phase(boseiju_lexer::terminal::Phase),
}

impl Node for StepOrPhase {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::StepOrPhase.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Step(child) => children.push(child as &dyn Node),
            Self::Phase(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StepOrPhase {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Step(child) => child.span(),
            Self::Phase(child) => child.span(),
        }
    }
}

impl Default for StepOrPhase {
    fn default() -> Self {
        Self::Step(Default::default())
    }
}
