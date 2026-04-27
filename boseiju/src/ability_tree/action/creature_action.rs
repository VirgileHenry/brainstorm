mod creature_attacks_action;
mod creature_blocks_action;
mod creature_deals_damage_action;
mod creature_dies_action;

pub use creature_attacks_action::CreatureAttacksAction;
pub use creature_blocks_action::CreatureBlocksAction;
pub use creature_deals_damage_action::CreatureDealsDamageAction;
pub use creature_dies_action::CreatureDiesAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An action a creature can perform.
///
/// These are the actions ONLY creatures can perform, there might be more actions
/// that can be performed by all permanents under the [`PermanentAction`] node.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureAction {
    Attacks(CreatureAttacksAction),
    Blocks(CreatureBlocksAction),
    DealsDamage(CreatureDealsDamageAction),
    Dies(CreatureDiesAction),
}

impl crate::ability_tree::AbilityTreeNode for CreatureAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Blocks(child) => children.push(child as &dyn AbilityTreeNode),
            Self::DealsDamage(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Dies(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature action:")?;
        out.push_final_branch()?;
        match self {
            Self::Attacks(event) => event.display(out)?,
            Self::Blocks(event) => event.display(out)?,
            Self::DealsDamage(event) => event.display(out)?,
            Self::Dies(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature action"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacks(child) => child.node_span(),
            Self::Blocks(child) => child.node_span(),
            Self::DealsDamage(child) => child.node_span(),
            Self::Dies(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureAction {
    fn dummy_init() -> Self {
        Self::Attacks(crate::utils::dummy())
    }
}
