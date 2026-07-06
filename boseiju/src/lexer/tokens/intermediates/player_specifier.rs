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
    Player {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },

    /* Opponent designation */
    Opponent {
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
    ActivePlayer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DefendingPlayer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NearestOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartingPlayer {
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
    Voter {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PlayerSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::You { span } => *span,
            Self::Player { span } => *span,
            Self::Opponent { span } => *span,
            Self::Controller { span } => *span,
            Self::Owner { span } => *span,
            Self::ActivePlayer { span } => *span,
            Self::DefendingPlayer { span } => *span,
            Self::NearestOpponent { span } => *span,
            Self::StartingPlayer { span } => *span,
            Self::TheMonarch { span } => *span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
            Self::Voter { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for PlayerSpecifier {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            /* You */
            "you" | "yourself" => Some(Self::You {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Players designation */
            "player" | "players" => Some(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Opponent designation */
            "opponent" | "opponents" => Some(Self::Opponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Object related players */
            "controller" | "controllers" | "controllers'" => Some(Self::Controller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owner" | "owners" | "owners'" => Some(Self::Owner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Context related players */
            "active player" => Some(Self::ActivePlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "defending player" => Some(Self::DefendingPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nearest opponent" => Some(Self::NearestOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting player" => Some(Self::StartingPlayer {
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
            "voter" => Some(Self::Voter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
