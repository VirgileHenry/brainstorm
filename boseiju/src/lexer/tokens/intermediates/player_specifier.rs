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
    Controller {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DefendingPlayer {
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
    Owner {
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
            Self::Controller { span } => *span,
            Self::DefendingPlayer { span } => *span,
            Self::EachOpponent { span } => *span,
            Self::Owner { span } => *span,
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
            "each player" | "players" => Some(Self::All {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an opponent" | "a player" => Some(Self::Any {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "controller" => Some(Self::Controller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "defending player" => Some(Self::DefendingPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "each opponent" | "opponents" | "your opponents" => Some(Self::EachOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owner" => Some(Self::Owner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target opponent" | "target player" | "target players" => Some(Self::TargetOpponent {
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
