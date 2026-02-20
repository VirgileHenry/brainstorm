use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Charasteristic defining abilities, from the comprehensive rules:
///
/// A kind of static ability that conveys information about an object’s characteristics
/// that would normally be found elsewhere on that object (such as in its mana cost,
/// type line, or power/toughness box). See rule 604.3.
///
/// See https://mtg.fandom.com/wiki/Characteristic-defining_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CharacteristicDefiningAbility {}

impl AbilityTreeNode for CharacteristicDefiningAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CharacteristicDefiningAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let children = arrayvec::ArrayVec::new_const();
        match self {
            _ => {}
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "characteristic defining ability:")?;
        out.push_final_branch()?;
        match self {
            _ => {}
        }
        out.pop_branch();
        Ok(())
    }
}
