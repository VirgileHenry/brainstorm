mod object_controller;
mod object_owner;
mod specified_player;
mod you;

pub mod player_specifier;

pub use object_controller::ObjectController;
pub use object_owner::ObjectOwner;
pub use specified_player::SpecifiedPlayer;
pub use you::You;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    ObjectController(ObjectController),
    ObjectOwner(ObjectOwner),
    SpecifiedPlayer(SpecifiedPlayer<Q>),
    You(You),
}

impl<Q> Node for PlayerReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerReference
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ObjectController(child) => children.push(child as &dyn Node),
            Self::ObjectOwner(child) => children.push(child as &dyn Node),
            Self::SpecifiedPlayer(child) => children.push(child as &dyn Node),
            Self::You(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player reference:")?;
        out.push_final_branch()?;
        match self {
            Self::ObjectController(child) => child.display(out)?,
            Self::ObjectOwner(child) => child.display(out)?,
            Self::SpecifiedPlayer(child) => child.display(out)?,
            Self::You(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for PlayerReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ObjectController(child) => child.span(),
            Self::ObjectOwner(child) => child.span(),
            Self::SpecifiedPlayer(child) => child.span(),
            Self::You(child) => child.span(),
        }
    }
}

impl<Q> Default for PlayerReference<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn default() -> Self {
        Self::You(Default::default())
    }
}

pub type ActivePlayerReference = PlayerReference<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type PassivePlayerReference = PlayerReference<crate::ability_tree::quantifier::PassiveQuantifier>;
