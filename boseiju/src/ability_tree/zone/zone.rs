use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnableZone {
    Graveyard {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Hand {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Library {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl OwnableZone {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Graveyard { span } => *span,
            Self::Hand { span } => *span,
            Self::Library { span } => *span,
        }
    }
}

impl AbilityTreeNode for OwnableZone {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::OwnableZoneIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::OwnableZone(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "zone"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Graveyard { span } => *span,
            Self::Hand { span } => *span,
            Self::Library { span } => *span,
        }
    }
}

impl std::fmt::Display for OwnableZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnableZone::Graveyard { .. } => write!(f, "graveyard"),
            OwnableZone::Hand { .. } => write!(f, "hand"),
            OwnableZone::Library { .. } => write!(f, "library"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for OwnableZone {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "graveyard" => Some(OwnableZone::Graveyard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hand" => Some(OwnableZone::Hand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "library" => Some(OwnableZone::Library {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnableZone {
    fn dummy_init() -> Self {
        Self::Library {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
