use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A recurrent instant is an instant that appear in a recurrent manner.
/// For instance, "the beginning of your turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurrentInstant {
    pub step_or_phase: crate::ability_tree::time::StepOrPhase,
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for RecurrentInstant {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::RecurrentInstant.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.step_or_phase as &dyn AbilityTreeNode);
        children.push(&self.owner as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for RecurrentInstant {
    fn dummy_init() -> Self {
        Self {
            step_or_phase: crate::utils::dummy(),
            owner: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
