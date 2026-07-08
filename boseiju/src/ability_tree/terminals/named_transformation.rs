use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedTransformation {
    EverflameHeroesLegacy {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fenric {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HumbleMerchant {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LegitimateBuisnessperson {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MilevaTheStalwart {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Moon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VituGhazi {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedTransformation {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedTransformationIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedTransformation(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named transformation"
    }
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for NamedTransformation {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::EverflameHeroesLegacy { span } => *span,
            Self::Fenric { span } => *span,
            Self::HumbleMerchant { span } => *span,
            Self::LegitimateBuisnessperson { span } => *span,
            Self::MilevaTheStalwart { span } => *span,
            Self::Moon { span } => *span,
            Self::VituGhazi { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedTransformation::EverflameHeroesLegacy { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::Fenric { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::HumbleMerchant { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::LegitimateBuisnessperson { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::MilevaTheStalwart { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::Moon { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::VituGhazi { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedTransformation {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "everflame, heroes' legacy" => Some(NamedTransformation::EverflameHeroesLegacy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fenric" => Some(NamedTransformation::Fenric {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "humble merchant" => Some(NamedTransformation::HumbleMerchant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legitimate businessperson" => Some(NamedTransformation::LegitimateBuisnessperson {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mileva, the stalwart" => Some(NamedTransformation::MilevaTheStalwart {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "moon" => Some(NamedTransformation::Moon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vitu-ghazi" => Some(NamedTransformation::VituGhazi {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
