use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerSpecifier {
    All {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EachOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TargetOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ToYourLeft {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ToYourRight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    You {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PlayerSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::All { span } => *span,
            Self::Any { span } => *span,
            Self::EachOpponent { span } => *span,
            Self::TargetOpponent { span } => *span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
            Self::You { span } => *span,
        }
    }
}

impl AbilityTreeNode for PlayerSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::PlayerSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::PlayerSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "player specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::All { span } => *span,
            Self::Any { span } => *span,
            Self::EachOpponent { span } => *span,
            Self::TargetOpponent { span } => *span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
            Self::You { span } => *span,
        }
    }
}

impl IntoToken for PlayerSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "each player" => Some(Self::All {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an opponent" | "a player" => Some(Self::Any {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "each opponent" | "opponents" | "your opponents" => Some(Self::EachOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target opponent" => Some(Self::TargetOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the player to your left" => Some(Self::ToYourLeft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the player to your right" => Some(Self::ToYourRight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you" => Some(Self::You {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for PlayerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All { .. } => write!(f, "all players"),
            Self::Any { .. } => write!(f, "a player"),
            Self::EachOpponent { .. } => write!(f, "each opponent"),
            Self::TargetOpponent { .. } => write!(f, "target opponent"),
            Self::ToYourLeft { .. } => write!(f, "the player to your left"),
            Self::ToYourRight { .. } => write!(f, "the player to your right"),
            Self::You { .. } => write!(f, "you"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerSpecifier {
    fn dummy_init() -> Self {
        Self::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
