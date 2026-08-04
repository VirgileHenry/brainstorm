mod add_mana_imperative;
mod change_zone_imperative;
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
mod lose_life_imperative;
mod modal_imperative;
mod pay_life_imperative;
mod pay_mana_imperative;
mod put_counters_imperative;
mod remove_counters_imperative;
mod sacrifice_imperative;
mod search_imperative;
mod tap_imperative;
mod untap_imperative;

pub use add_mana_imperative::*;
pub use change_zone_imperative::*;
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
pub use lose_life_imperative::*;
pub use modal_imperative::*;
pub use pay_life_imperative::*;
pub use pay_mana_imperative::*;
pub use put_counters_imperative::*;
pub use remove_counters_imperative::*;
pub use sacrifice_imperative::*;
pub use search_imperative::*;
pub use tap_imperative::*;
pub use untap_imperative::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
    pub executing_player: crate::ability_tree::player::PlayerReference,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for Imperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Imperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        children.push(&self.executing_player as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Imperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for Imperative {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            executing_player: Default::default(),
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
    DealsDamage(DealsDamageImperative),
    Draw(DrawImperative),
    ForEach(ForEachImperative),
    GainLife(GainLifeImperative),
    GenerateContinuousEffect(GenerateContinuousEffectImperative),
    GenerateDelayedTriggeredAbility(GenerateDelayedTriggeredAbilityImperative),
    KeywordAction(KeywordAction),
    LoseLife(LoseLifeImperative),
    Modal(ModalImperative),
    PayLife(PayLifeImperative),
    PayMana(PayManaImperative),
    PutCounters(PutCountersImperative),
    RemoveCounters(RemoveCountersImperative),
}

impl Node for ImperativeKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ImperativeKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AddMana(child) => children.push(child as &dyn Node),
            Self::ChangeZone(child) => children.push(child as &dyn Node),
            Self::DealsDamage(child) => children.push(child as &dyn Node),
            Self::Draw(child) => children.push(child as &dyn Node),
            Self::ForEach(child) => children.push(child as &dyn Node),
            Self::GainLife(child) => children.push(child as &dyn Node),
            Self::GenerateContinuousEffect(child) => children.push(child as &dyn Node),
            Self::GenerateDelayedTriggeredAbility(child) => children.push(child as &dyn Node),
            Self::KeywordAction(child) => children.push(child as &dyn Node),
            Self::LoseLife(child) => children.push(child as &dyn Node),
            Self::Modal(child) => children.push(child as &dyn Node),
            Self::PayLife(child) => children.push(child as &dyn Node),
            Self::PayMana(child) => children.push(child as &dyn Node),
            Self::PutCounters(child) => children.push(child as &dyn Node),
            Self::RemoveCounters(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative kind:")?;
        out.push_final_branch()?;
        match self {
            Self::AddMana(imperative) => imperative.display(out)?,
            Self::ChangeZone(imperative) => imperative.display(out)?,
            Self::DealsDamage(imperative) => imperative.display(out)?,
            Self::Draw(imperative) => imperative.display(out)?,
            Self::ForEach(imperative) => imperative.display(out)?,
            Self::GainLife(imperative) => imperative.display(out)?,
            Self::GenerateContinuousEffect(imperative) => imperative.display(out)?,
            Self::GenerateDelayedTriggeredAbility(imperative) => imperative.display(out)?,
            Self::KeywordAction(imperative) => imperative.display(out)?,
            Self::LoseLife(imperative) => imperative.display(out)?,
            Self::Modal(imperative) => imperative.display(out)?,
            Self::PayLife(imperative) => imperative.display(out)?,
            Self::PayMana(imperative) => imperative.display(out)?,
            Self::PutCounters(imperative) => imperative.display(out)?,
            Self::RemoveCounters(imperative) => imperative.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ImperativeKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AddMana(child) => child.span(),
            Self::ChangeZone(child) => child.span(),
            Self::DealsDamage(child) => child.span(),
            Self::Draw(child) => child.span(),
            Self::ForEach(child) => child.span(),
            Self::GainLife(child) => child.span(),
            Self::GenerateContinuousEffect(child) => child.span(),
            Self::GenerateDelayedTriggeredAbility(child) => child.span(),
            Self::KeywordAction(child) => child.span(),
            Self::LoseLife(child) => child.span(),
            Self::Modal(child) => child.span(),
            Self::PayLife(child) => child.span(),
            Self::PayMana(child) => child.span(),
            Self::PutCounters(child) => child.span(),
            Self::RemoveCounters(child) => child.span(),
        }
    }
}

impl Default for ImperativeKind {
    fn default() -> Self {
        Self::AddMana(Default::default())
    }
}
