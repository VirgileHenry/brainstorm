use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PowerToughnessModifier {
    pub modifier: crate::ability_tree::terminals::PowerToughnessModifier,
}

impl AbilityTreeNode for PowerToughnessModifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectObjectGainsAbilies.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.modifier as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "power toughness modifier:")?;
        out.push_inter_branch()?;
        write!(out, "modifiers:")?;
        out.push_final_branch()?;
        self.modifier.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PowerToughnessModifier {
    fn dummy_init() -> Self {
        Self {
            modifier: crate::utils::dummy(),
        }
    }
}
