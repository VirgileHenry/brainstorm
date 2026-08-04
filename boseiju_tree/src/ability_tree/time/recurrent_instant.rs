use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A recurrent instant is an instant that appear in a recurrent manner.
/// For instance, "the beginning of your turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurrentInstant {
    pub step_or_phase: crate::ability_tree::time::StepOrPhase,
    pub owner: crate::ability_tree::player::PlayerReference,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for RecurrentInstant {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::RecurrentInstant.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn Node);
        children.push(&self.owner as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "recurrent instant:")?;
        out.push_inter_branch()?;
        self.step_or_phase.display(out)?;
        out.next_final_branch()?;
        write!(out, "of player:")?;
        out.push_final_branch()?;
        self.owner.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "recurrent instant"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for RecurrentInstant {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for RecurrentInstant {
    fn default() -> Self {
        Self {
            step_or_phase: Default::default(),
            owner: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
