pub mod form;

pub mod add_mana;
pub mod attack;
pub mod cast;
pub mod deal_damages;
pub mod destroy;
pub mod draw;
pub mod etb;
pub mod pay_life;
pub mod pay_mana;
pub mod put_counters;
pub mod sacrifice;
pub mod tap;
pub mod untap;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A deed is an elementary thing that happens in a game.
///
/// It can either be seen as an event to be listened to, or as an instruction for the player.
/// The idea of the deed node is to regroup both passive and active form of events, to avoid repetition.
///
/// The deed node is generic over the form, so that they both exist.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deed<F>
where
    F: form::DeedForm,
{
    AddMana(add_mana::AddMana<F::Quantifier>),
    Attack(attack::Attack<F::Quantifier>),
    Cast(cast::Cast<F::Quantifier>),
    DealDamages(deal_damages::DealDamages),
    Destroy(destroy::Destroy<F::Quantifier>),
    Draw(draw::Draw<F::Quantifier>),
    Etb(etb::EntersTheBattlefield),
    PayLife(pay_life::PayLife),
    PayMana(pay_mana::PayMana),
    PutCounters(put_counters::PutCounters<F::Quantifier>),
    Sacrifice(sacrifice::Sacrifice<F::Quantifier>),
    Tap(tap::Tap<F::Quantifier>),
}

impl<F> crate::Node for Deed<F>
where
    F: form::DeedForm,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Deed
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AddMana(child) => children.push(child as &dyn Node),
            Self::Attack(child) => children.push(child as &dyn Node),
            Self::Cast(child) => children.push(child as &dyn Node),
            Self::DealDamages(child) => children.push(child as &dyn Node),
            Self::Destroy(child) => children.push(child as &dyn Node),
            Self::Draw(child) => children.push(child as &dyn Node),
            Self::Etb(child) => children.push(child as &dyn Node),
            Self::PayLife(child) => children.push(child as &dyn Node),
            Self::PayMana(child) => children.push(child as &dyn Node),
            Self::PutCounters(child) => children.push(child as &dyn Node),
            Self::Sacrifice(child) => children.push(child as &dyn Node),
            Self::Tap(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deed ({}):", F::FORM_NAME)?;
        out.push_final_branch()?;
        match self {
            Self::AddMana(child) => child.display(out)?,
            Self::Attack(child) => child.display(out)?,
            Self::Cast(child) => child.display(out)?,
            Self::DealDamages(child) => child.display(out)?,
            Self::Destroy(child) => child.display(out)?,
            Self::Draw(child) => child.display(out)?,
            Self::Etb(child) => child.display(out)?,
            Self::PayLife(child) => child.display(out)?,
            Self::PayMana(child) => child.display(out)?,
            Self::PutCounters(child) => child.display(out)?,
            Self::Sacrifice(child) => child.display(out)?,
            Self::Tap(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "deed"
    }
}

#[cfg(feature = "spanned_tree")]
impl<F> boseiju_span::Spanned for Deed<F>
where
    F: form::DeedForm,
{
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AddMana(child) => child.span(),
            Self::Attack(child) => child.span(),
            Self::Cast(child) => child.span(),
            Self::DealDamages(child) => child.span(),
            Self::Destroy(child) => child.span(),
            Self::Draw(child) => child.span(),
            Self::Etb(child) => child.span(),
            Self::PayLife(child) => child.span(),
            Self::PayMana(child) => child.span(),
            Self::PutCounters(child) => child.span(),
            Self::Sacrifice(child) => child.span(),
            Self::Tap(child) => child.span(),
        }
    }
}

impl<F> Default for Deed<F>
where
    F: form::DeedForm,
{
    fn default() -> Self {
        Self::Cast(Default::default())
    }
}

pub type ActiveFormDeed = Deed<form::ActiveForm>;
pub type PassiveFormDeed = Deed<form::PassiveForm>;
