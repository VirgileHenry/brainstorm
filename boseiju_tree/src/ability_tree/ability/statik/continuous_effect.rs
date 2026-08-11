pub mod continuous_effect_kind;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A continuous effect, from the comprehensive rules:
///
/// An effect that modifies characteristics of objects,
/// modifies control of objects, or affects players or the rules of the game,
/// for a fixed or indefinite period. See rule 611, “Continuous Effects”.
///
/// See also <https://mtg.fandom.com/wiki/Continuous_effect>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub effect: continuous_effect_kind::ContinuousEffectKind,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ContinuousEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ContinuousEffect
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.effect as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "continuous effect:")?;
        out.push_final_branch()?;
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ContinuousEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ContinuousEffect {
    fn default() -> Self {
        Self {
            effect: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
