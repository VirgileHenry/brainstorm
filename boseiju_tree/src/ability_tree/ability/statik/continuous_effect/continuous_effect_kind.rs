mod modify_object;
mod modify_rules;

pub use modify_object::*;
pub use modify_rules::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::replacement_effect::ReplacementEffect;

/// All kinds of continuous effects
///
/// See also <https://mtg.fandom.com/wiki/Continuous_effect>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContinuousEffectKind {
    ModifyRule(ModifyRuleEffect),
    ModifyObjectAbilities(ModifyObjectEffect),
    ReplacementEffect(ReplacementEffect),
}

impl Node for ContinuousEffectKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ContinuousEffectKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ModifyRule(child) => children.push(child as &dyn Node),
            Self::ModifyObjectAbilities(child) => children.push(child as &dyn Node),
            Self::ReplacementEffect(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "continuous effect kind")?;
        out.push_final_branch()?;
        match self {
            Self::ModifyRule(child) => child.display(out)?,
            Self::ModifyObjectAbilities(child) => child.display(out)?,
            Self::ReplacementEffect(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "continuous effect kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ContinuousEffectKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ModifyRule(child) => child.span(),
            Self::ModifyObjectAbilities(child) => child.span(),
            Self::ReplacementEffect(child) => child.span(),
        }
    }
}

impl Default for ContinuousEffectKind {
    fn default() -> Self {
        Self::ModifyObjectAbilities(Default::default())
    }
}
