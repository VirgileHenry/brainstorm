mod object_gains_abilities;
mod power_toughness_modifier;
mod replacement_effect;

pub use object_gains_abilities::ContinuousEffectObjectGainsAbilies;
pub use replacement_effect::ContinuousEffectReplacementEvent;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// All kinds of continuous effects, as per continuous effects in
/// https://mtg.fandom.com/wiki/Continuous_effect
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ContinuousEffectKind {
    ObjectGainsAbilies(ContinuousEffectObjectGainsAbilies),
    ReplacementEffect(ContinuousEffectReplacementEvent),
}

impl AbilityTreeNode for ContinuousEffectKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ObjectGainsAbilies(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ReplacementEffect(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "continuous effect kind")?;
        out.push_final_branch()?;
        match self {
            Self::ObjectGainsAbilies(child) => child.display(out)?,
            Self::ReplacementEffect(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffectKind {
    fn dummy_init() -> Self {
        Self::ObjectGainsAbilies(crate::utils::dummy())
    }
}
