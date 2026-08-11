pub mod form;

pub mod cast;
// pub mod pay_mana;

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
    Cast(cast::Cast<F>),
    // PayMana(pay_mana::PayMana<F>),
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
            Self::Cast(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deed ({}):", F::FORM_NAME)?;
        out.push_final_branch()?;
        match self {
            Self::Cast(child) => child.display(out)?,
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
            Self::Cast(child) => child.span(),
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
