mod continuous_effect_kind;

pub use continuous_effect_kind::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A continuous effect, from the comprehensive rules:
///
/// An effect that modifies characteristics of objects,
/// modifies control of objects, or affects players or the rules of the game,
/// for a fixed or indefinite period. See rule 611, “Continuous Effects”.
///
/// See https://mtg.fandom.com/wiki/Continuous_effect.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContinuousEffect {
    pub effect: continuous_effect_kind::ContinuousEffectKind,
    pub duration: crate::ability_tree::time::ForwardDuration,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ContinuousEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn AbilityTreeNode);
        children.push(&self.duration as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "continuous effect:")?;
        out.push_inter_branch()?;
        write!(out, "duration: {}", self.duration)?;
        out.next_final_branch()?;
        write!(out, "effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "continuous effect"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffect {
    fn dummy_init() -> Self {
        Self {
            effect: crate::utils::dummy(),
            duration: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
