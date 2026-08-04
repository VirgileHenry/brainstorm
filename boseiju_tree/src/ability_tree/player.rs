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
pub enum PlayerReference {
    ObjectController(ObjectController),
    ObjectOwner(ObjectOwner),
    SpecifiedPlayer(SpecifiedPlayer),
    You(You),
}

impl Node for PlayerReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerReference.id()
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
impl boseiju_span::Spanned for PlayerReference {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ObjectController(child) => child.span(),
            Self::ObjectOwner(child) => child.span(),
            Self::SpecifiedPlayer(child) => child.span(),
            Self::You(child) => child.span(),
        }
    }
}

impl Default for PlayerReference {
    fn default() -> Self {
        Self::You(Default::default())
    }
}
