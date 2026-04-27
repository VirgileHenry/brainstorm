mod modify_object;
mod modify_rules;

pub use modify_object::*;
pub use modify_rules::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
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

impl AbilityTreeNode for ContinuousEffectKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ModifyRule(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ModifyObjectAbilities(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ReplacementEffect(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ModifyRule(child) => child.node_span(),
            Self::ModifyObjectAbilities(child) => child.node_span(),
            Self::ReplacementEffect(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffectKind {
    fn dummy_init() -> Self {
        Self::ModifyObjectAbilities(crate::utils::dummy())
    }
}
