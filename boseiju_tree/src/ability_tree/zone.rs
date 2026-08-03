mod owned_zone;

use idris::Idris;
pub use owned_zone::OwnedZone;

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
    Anywhere {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exile {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OwnedZone(OwnedZone),
    TheBattlefield {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Node for ZoneReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ZoneReferenceIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OwnedZone(child) => children.push(child as &dyn Node),
            Self::Anywhere { .. } | Self::Exile { .. } | Self::TheBattlefield { .. } => children.push(
                crate::dummy_terminal::TreeNodeDummyTerminal::new(crate::NodeKind::ZoneReference(self.clone()).id()) as &dyn Node,
            ),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::Anywhere { .. } => write!(out, "anywhere"),
            Self::Exile { .. } => write!(out, "exile"),
            Self::OwnedZone(owned) => owned.display(out),
            Self::TheBattlefield { .. } => write!(out, "the battlefield"),
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
            Self::Anywhere { span } => *span,
            Self::Exile { span } => *span,
            Self::OwnedZone(child) => child.span(),
            Self::TheBattlefield { span } => *span,
        }
    }
}

impl Default for ZoneReference {
    fn default() -> Self {
        Self::Anywhere {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
