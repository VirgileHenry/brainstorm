use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerSpecifier {
    /* You designation */
    You {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },

    /* Players designation */
    AllPlayers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AnyPlayer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TargetPlayer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },

    /* Opponent designation */
    AllOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AnotherOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AnyOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TargetOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThatOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },

    /* Object related players */
    Controller {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Owner {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },

    /* Context related players */
    DefendingPlayer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheMonarch {
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
}

#[cfg(feature = "spanned_tree")]
impl PlayerSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::You { span } => *span,
            Self::AllPlayers { span } => *span,
            Self::AnyPlayer { span } => *span,
            Self::TargetPlayer { span } => *span,
            Self::AllOpponent { span } => *span,
            Self::AnotherOpponent { span } => *span,
            Self::AnyOpponent { span } => *span,
            Self::TargetOpponent { span } => *span,
            Self::ThatOpponent { span } => *span,
            Self::Controller { span } => *span,
            Self::Owner { span } => *span,
            Self::DefendingPlayer { span } => *span,
            Self::TheMonarch { span } => *span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for PlayerSpecifier {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            /* You */
            "you" => Some(Self::You {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Players designation */
            "each player" | "players" => Some(Self::AllPlayers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "a player" => Some(Self::AnyPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target player" | "target players" => Some(Self::TargetPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Opponent designation */
            "each opponent" | "opponents" | "your opponents" => Some(Self::AllOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "another opponent" => Some(Self::AnotherOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an opponent" | "any opponent" => Some(Self::AnyOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target opponent" | "target opponents" => Some(Self::TargetOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "that opponent" => Some(Self::ThatOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Object related players */
            "controller" | "controllers'" => Some(Self::Controller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owner" | "owners'" => Some(Self::Owner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Context related players */
            "defending player" => Some(Self::DefendingPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the monarch" => Some(Self::TheMonarch {
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
            _ => None,
        }
    }
}
