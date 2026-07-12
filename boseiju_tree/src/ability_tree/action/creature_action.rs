mod creature_attacks_action;
mod creature_blocks_action;
mod creature_deals_damage_action;
mod creature_dies_action;

pub use creature_attacks_action::CreatureAttacksAction;
pub use creature_blocks_action::CreatureBlocksAction;
pub use creature_deals_damage_action::CreatureDealsDamageAction;
pub use creature_dies_action::CreatureDiesAction;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An action a creature can perform.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureAction {
    Attacks(CreatureAttacksAction),
    Blocks(CreatureBlocksAction),
    DealsDamage(CreatureDealsDamageAction),
    Dies(CreatureDiesAction),
}

impl crate::Node for CreatureAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn Node),
            Self::Blocks(child) => children.push(child as &dyn Node),
            Self::DealsDamage(child) => children.push(child as &dyn Node),
            Self::Dies(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureAction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Attacks(child) => child.span(),
            Self::Blocks(child) => child.span(),
            Self::DealsDamage(child) => child.span(),
            Self::Dies(child) => child.span(),
        }
    }
}


impl Default for CreatureAction {
    fn default() -> Self {
        Self::Attacks(Default::default())
    }
}
