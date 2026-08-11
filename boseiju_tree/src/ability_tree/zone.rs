mod anywhere;
mod exile;
mod owned_zone;
mod the_battlefield;

pub use anywhere::Anywhere;
pub use exile::Exile;
pub use owned_zone::OwnedZone;
pub use the_battlefield::TheBattlefield;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Reference to a "zone", which are the various places of the game.
///
/// Some references are to zone that are common to all players: exile, the battlefield, etc.
/// Otherwise, there are "owned zones" such as the players hand, libraries, etc.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneReference {
    Anywhere(Anywhere),
    Exile(Exile),
    OwnedZone(OwnedZone),
    TheBattlefield(TheBattlefield),
}

impl Node for ZoneReference {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ZoneReference
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Anywhere(child) => children.push(child as &dyn Node),
            Self::Exile(child) => children.push(child as &dyn Node),
            Self::OwnedZone(child) => children.push(child as &dyn Node),
            Self::TheBattlefield(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Anywhere(owned) => owned.display(out),
            Self::Exile(owned) => owned.display(out),
            Self::OwnedZone(owned) => owned.display(out),
            Self::TheBattlefield(owned) => owned.display(out),
        }
    }

    fn node_tag(&self) -> &'static str {
        "zone reference type"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ZoneReference {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Anywhere(child) => child.span(),
            Self::Exile(child) => child.span(),
            Self::OwnedZone(child) => child.span(),
            Self::TheBattlefield(child) => child.span(),
        }
    }
}

impl Default for ZoneReference {
    fn default() -> Self {
        Self::Anywhere(Default::default())
    }
}
