mod choose_imperative;
mod create_token_imperative;
mod deals_damage_imperative;
mod destroy_imperative;
mod exile_imperative;
mod put_counters_imperative;
mod return_imperative;
mod sacrifice_imperative;

pub use choose_imperative::ChooseImperative;
pub use create_token_imperative::CreateTokenImperative;
pub use create_token_imperative::CreatedTokenKind;
pub use create_token_imperative::TokenCreation;
pub use deals_damage_imperative::DealsDamageImperative;
pub use destroy_imperative::DestroyImperative;
pub use exile_imperative::ExileFollowUp;
pub use exile_imperative::ExileFollowUpReturn;
pub use exile_imperative::ExileImperative;
pub use put_counters_imperative::CounterKind;
pub use put_counters_imperative::CounterOnPermanent;
pub use put_counters_imperative::PutCountersImperative;
pub use return_imperative::ReturnImperative;
pub use sacrifice_imperative::SacrificeImperative;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An imperative is an instruction a player must follow.
/// It represents something that shall be done, and can appear in many places:
/// In spell / ability resolution, in costs, etc.
///
/// Imperatives regroups a lot of what "can be done" in the game: draw cards,
/// destroy things, move cards around, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Imperative {
    Choose(ChooseImperative),
    CreateToken(CreateTokenImperative),
    DealsDamage(DealsDamageImperative),
    Destroy(DestroyImperative),
    Exile(ExileImperative),
    Put(PutCountersImperative),
    Return(ReturnImperative),
    Sacrifice(SacrificeImperative),
}

impl AbilityTreeNode for Imperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Imperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Choose(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CreateToken(child) => children.push(child as &dyn AbilityTreeNode),
            Self::DealsDamage(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Destroy(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Exile(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Put(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Return(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Sacrifice(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative:")?;
        out.push_final_branch()?;
        match self {
            Imperative::Choose(imperative) => imperative.display(out)?,
            Imperative::CreateToken(imperative) => imperative.display(out)?,
            Imperative::DealsDamage(imperative) => imperative.display(out)?,
            Imperative::Destroy(imperative) => imperative.display(out)?,
            Imperative::Exile(imperative) => imperative.display(out)?,
            Imperative::Put(imperative) => imperative.display(out)?,
            Imperative::Return(imperative) => imperative.display(out)?,
            Imperative::Sacrifice(imperative) => imperative.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Imperative {
    fn dummy_init() -> Self {
        Self::Destroy(crate::utils::dummy())
    }
}
