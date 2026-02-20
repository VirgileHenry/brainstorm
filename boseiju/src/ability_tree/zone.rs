mod owned_zone;
mod zone;

use idris::Idris;
pub use owned_zone::OwnedZone;
pub use zone::OwnableZone;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Reference to a "zone", which are the various places of the game.
///
/// Some references are to zone that are common to all players: exile, the battlefield, etc.
/// Otherwise, there are "owned zones" such as the players hand, libraries, etc.
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ZoneReference {
    Anywhere,
    Exile,
    OwnedZone(OwnedZone),
    TheBattlefield,
}

impl AbilityTreeNode for ZoneReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ZoneReferenceIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OwnedZone(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Anywhere | Self::Exile | Self::TheBattlefield => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                    crate::ability_tree::NodeKind::ZoneReference(self.clone()).id(),
                ) as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ZoneReference::Anywhere => write!(out, "anywhere"),
            ZoneReference::Exile => write!(out, "exile"),
            ZoneReference::OwnedZone(owned) => owned.display(out),
            ZoneReference::TheBattlefield => write!(out, "the battlefield"),
        }
    }
}
