use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A spell ability.
///
/// From the comprehensive rules 113.3a:
/// Spell abilities are abilities that are followed as instructions while an
/// instant or sorcery spell is resolving. Any text on an instant or sorcery spell
/// is a spell ability unless it's an activated ability, a triggered ability,
/// or a static ability that fits the criteria described in rule 113.6.
///
/// Spell abilities are represented as a list of statements that are all of the spell effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpellAbility {
    pub effects: arrayvec::ArrayVec<crate::ability_tree::statement::Statement, MAX_CHILDREN_PER_NODE>,
}

impl crate::ability_tree::AbilityTreeNode for SpellAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpellAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.effects.iter() {
            children.push(child as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell ability:")?;
        for effect in self.effects.iter().take(self.effects.len().saturating_sub(1)) {
            effect.display(out)?;
        }
        if let Some(effect) = self.effects.last() {
            effect.display(out)?;
        }
        Ok(())
    }
}
