pub mod alterative_casting_permissions;
pub mod continuous_effect;
pub mod cost_modification_effect;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A static ability, from the comprehensive rules:
///
/// A kind of ability.
/// Static abilities do something all the time rather than being activated or triggered.
/// See rule 113, “Abilities”, and rule 604, “Handling Static Abilities”.
///
/// See the MTG wiki: https://mtg.fandom.com/wiki/Static_ability
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct StaticAbility {
    pub kind: StaticAbilityKind,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
}

impl AbilityTreeNode for StaticAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StaticAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn AbilityTreeNode);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn AbilityTreeNode),
            None => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability:")?;
        out.push_inter_branch()?;
        self.kind.display(out)?;
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "if condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StaticAbility {
    fn dummy_init() -> Self {
        Self {
            kind: crate::utils::dummy(),
            condition: None,
        }
    }
}

/// The kind of a static ability.
///
/// All of the different static abilities that there is.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum StaticAbilityKind {
    ContinuousEffect(continuous_effect::ContinuousEffect),
    CostModificationEffect(cost_modification_effect::CostModificationEffect),
    AlternativeCastingPermissions(alterative_casting_permissions::AlternativeCastingPermissions),
}

impl AbilityTreeNode for StaticAbilityKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::StaticAbilityKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ContinuousEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CostModificationEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::AlternativeCastingPermissions(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "static ability kind:")?;
        out.push_final_branch()?;
        match self {
            Self::ContinuousEffect(child) => child.display(out)?,
            Self::CostModificationEffect(child) => child.display(out)?,
            Self::AlternativeCastingPermissions(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StaticAbilityKind {
    fn dummy_init() -> Self {
        Self::ContinuousEffect(crate::utils::dummy())
    }
}
