mod add_mana_imperative;
mod choose_imperative;
mod create_token_imperative;
mod deals_damage_imperative;
mod destroy_imperative;
mod discard_imperative;
mod draw_imperative;
mod for_each_imperative;
mod gain_life;
mod generate_continuous_effect_imperative;
mod generate_delayed_trigger_ab_imperative;
mod keyword_action;
mod put_counters_imperative;
mod remove_counters_imperative;
mod return_imperative;
mod sacrifice_imperative;
mod search_imperative;
mod tap_imperative;
mod untap_imperative;

pub use add_mana_imperative::*;
pub use choose_imperative::*;
pub use create_token_imperative::*;
pub use deals_damage_imperative::*;
pub use destroy_imperative::*;
pub use discard_imperative::*;
pub use draw_imperative::*;
pub use for_each_imperative::*;
pub use gain_life::*;
pub use generate_continuous_effect_imperative::*;
pub use generate_delayed_trigger_ab_imperative::*;
pub use keyword_action::*;
pub use put_counters_imperative::*;
pub use remove_counters_imperative::*;
pub use return_imperative::*;
pub use sacrifice_imperative::*;
pub use search_imperative::*;
pub use tap_imperative::*;
pub use untap_imperative::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An imperative is an instruction a player must follow.
/// It represents something that shall be done, and can appear in many places:
/// In spell / ability resolution, in costs, etc.
///
/// Imperatives regroups a lot of what "can be done" in the game: draw cards,
/// destroy things, move cards around, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imperative {
    pub kind: ImperativeKind,
    pub executing_player: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for Imperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Imperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn AbilityTreeNode);
        children.push(&self.executing_player as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative:")?;
        out.push_inter_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "executing player:")?;
        out.push_final_branch()?;
        self.executing_player.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Imperative {
    fn dummy_init() -> Self {
        Self {
            kind: crate::utils::dummy(),
            executing_player: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// An imperative is an instruction a player must follow.
/// It represents something that shall be done, and can appear in many places:
/// In spell / ability resolution, in costs, etc.
///
/// Imperatives regroups a lot of what "can be done" in the game: draw cards,
/// destroy things, move cards around, etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImperativeKind {
    AddMana(AddManaImperative),
    ChangeZone(ChangeZoneImperative),
    CreateToken(CreateTokenImperative),
    DealsDamage(DealsDamageImperative),
    Destroy(DestroyImperative),
    Discard(DiscardImperative),
    Draw(DrawImperative),
    ForEach(ForEachImperative),
    GainLife(GainLifeImperative),
    GenerateContinuousEffect(GenerateContinuousEffectImperative),
    GenerateDelayedTriggeredAbility(GenerateDelayedTriggeredAbilityImperative),
    KeywordAction(KeywordAction),
    Modal(ModalImperative),
    PutCounters(PutCountersImperative),
    RemoveCounters(RemoveCountersImperative),
    Sacrifice(SacrificeImperative),
    Search(SearchImperative),
    Tap(TapImperative),
    Untap(UntapImperative),
}

impl AbilityTreeNode for ImperativeKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ImperativeKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AddMana(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ChangeZone(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CreateToken(child) => children.push(child as &dyn AbilityTreeNode),
            Self::DealsDamage(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Destroy(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Discard(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Draw(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ForEach(child) => children.push(child as &dyn AbilityTreeNode),
            Self::GainLife(child) => children.push(child as &dyn AbilityTreeNode),
            Self::GenerateContinuousEffect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::GenerateDelayedTriggeredAbility(child) => children.push(child as &dyn AbilityTreeNode),
            Self::KeywordAction(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Modal(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PutCounters(child) => children.push(child as &dyn AbilityTreeNode),
            Self::RemoveCounters(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Sacrifice(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Search(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Tap(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Untap(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative kind:")?;
        out.push_final_branch()?;
        match self {
            Self::AddMana(imperative) => imperative.display(out)?,
            Self::ChangeZone(imperative) => imperative.display(out)?,
            Self::CreateToken(imperative) => imperative.display(out)?,
            Self::DealsDamage(imperative) => imperative.display(out)?,
            Self::Destroy(imperative) => imperative.display(out)?,
            Self::Discard(imperative) => imperative.display(out)?,
            Self::Draw(imperative) => imperative.display(out)?,
            Self::ForEach(imperative) => imperative.display(out)?,
            Self::GainLife(imperative) => imperative.display(out)?,
            Self::GenerateContinuousEffect(imperative) => imperative.display(out)?,
            Self::GenerateDelayedTriggeredAbility(imperative) => imperative.display(out)?,
            Self::KeywordAction(imperative) => imperative.display(out)?,
            Self::Modal(imperative) => imperative.display(out)?,
            Self::PutCounters(imperative) => imperative.display(out)?,
            Self::RemoveCounters(imperative) => imperative.display(out)?,
            Self::Sacrifice(imperative) => imperative.display(out)?,
            Self::Search(imperative) => imperative.display(out)?,
            Self::Tap(imperative) => imperative.display(out)?,
            Self::Untap(imperative) => imperative.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AddMana(child) => child.node_span(),
            Self::ChangeZone(child) => child.node_span(),
            Self::CreateToken(child) => child.node_span(),
            Self::DealsDamage(child) => child.node_span(),
            Self::Destroy(child) => child.node_span(),
            Self::Discard(child) => child.node_span(),
            Self::Draw(child) => child.node_span(),
            Self::ForEach(child) => child.node_span(),
            Self::GainLife(child) => child.node_span(),
            Self::GenerateContinuousEffect(child) => child.node_span(),
            Self::GenerateDelayedTriggeredAbility(child) => child.node_span(),
            Self::KeywordAction(child) => child.node_span(),
            Self::Modal(child) => child.node_span(),
            Self::PutCounters(child) => child.node_span(),
            Self::RemoveCounters(child) => child.node_span(),
            Self::Sacrifice(child) => child.node_span(),
            Self::Search(child) => child.node_span(),
            Self::Tap(child) => child.node_span(),
            Self::Untap(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ImperativeKind {
    fn dummy_init() -> Self {
        Self::Destroy(crate::utils::dummy())
    }
}
