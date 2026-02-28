mod choose_imperative;
mod create_token_imperative;
mod deals_damage_imperative;
mod destroy_imperative;
mod discard_imperative;
mod draw_imperative;
mod exile_imperative;
mod gain_life;
mod put_counters_imperative;
mod remove_counters_imperative;
mod return_imperative;
mod sacrifice_imperative;

pub use choose_imperative::ChooseImperative;
pub use create_token_imperative::CreateTokenImperative;
pub use create_token_imperative::CreatedTokenKind;
pub use create_token_imperative::TokenCreation;
pub use deals_damage_imperative::DamagesDealt;
pub use deals_damage_imperative::DealsDamageImperative;
pub use destroy_imperative::DestroyImperative;
pub use discard_imperative::DiscardImperative;
pub use draw_imperative::DrawImperative;
pub use exile_imperative::ExileFollowUp;
pub use exile_imperative::ExileFollowUpReturn;
pub use exile_imperative::ExileImperative;
pub use gain_life::GainLifeImperative;
pub use put_counters_imperative::CounterKind;
pub use put_counters_imperative::CounterOnPermanent;
pub use put_counters_imperative::PutCountersImperative;
pub use remove_counters_imperative::RemovableCounterKind;
pub use remove_counters_imperative::RemovableCounterOnPermanent;
pub use remove_counters_imperative::RemoveCountersImperative;
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
    Discard(DiscardImperative),
    Draw(DrawImperative),
    Exile(ExileImperative),
    GainLife(GainLifeImperative),
    PutCounters(PutCountersImperative),
    RemoveCounters(RemoveCountersImperative),
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
            Self::Discard(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Draw(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Exile(child) => children.push(child as &dyn AbilityTreeNode),
            Self::GainLife(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PutCounters(child) => children.push(child as &dyn AbilityTreeNode),
            Self::RemoveCounters(child) => children.push(child as &dyn AbilityTreeNode),
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
            Imperative::Discard(imperative) => imperative.display(out)?,
            Imperative::Draw(imperative) => imperative.display(out)?,
            Imperative::Exile(imperative) => imperative.display(out)?,
            Imperative::GainLife(imperative) => imperative.display(out)?,
            Imperative::PutCounters(imperative) => imperative.display(out)?,
            Imperative::RemoveCounters(imperative) => imperative.display(out)?,
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

/// An imperative list is a list of imperative that should be executed.
///
/// The inner item is actually a conditional imperative, since there are list that contains
/// imperative and conditional ones. For example, Chart a Course states:
/// "Draw two cards. Then discard a card unless you attacked this turn."
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ImperativeList {
    pub executing_player: crate::ability_tree::terminals::PlayerSpecifier,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    pub imperatives: crate::utils::HeapArrayVec<Imperative, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for ImperativeList {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ImperativeList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.executing_player as &dyn AbilityTreeNode);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        for imperative in self.imperatives.iter() {
            children.push(imperative as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative list:")?;
        out.push_inter_branch()?;
        write!(out, "executing player:")?;
        self.executing_player.display(out)?;
        out.next_inter_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "if condition: none")?,
        }
        out.next_final_branch()?;
        write!(out, "imperatives:")?;
        for imperative in self.imperatives.iter().take(self.imperatives.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            imperative.display(out)?;
            out.pop_branch();
        }
        if let Some(imperative) = self.imperatives.last() {
            out.push_final_branch()?;
            imperative.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ImperativeList {
    fn dummy_init() -> Self {
        Self {
            executing_player: crate::utils::dummy(),
            imperatives: crate::utils::dummy(),
            condition: None,
        }
    }
}
