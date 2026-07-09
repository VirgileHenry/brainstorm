/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerSpecifier {
    /* You designation */
    You {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },

    /* Players designation */
    Player {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },

    /* Opponent designation */
    Opponent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },

    /* Object related players */
    Controller {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Owner {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },

    /* Context related players */
    ActivePlayer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DefendingPlayer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NearestOpponent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingPlayer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheMonarch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ToYourLeft {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ToYourRight {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Voter {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerSpecifier {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for PlayerSpecifier {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            /* You */
            "you" | "yourself" => Ok(Self::You {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Players designation */
            "player" | "players" => Ok(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Opponent designation */
            "opponent" | "opponents" => Ok(Self::Opponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Object related players */
            "controller" | "controllers" | "controllers'" => Ok(Self::Controller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owner" | "owners" | "owners'" => Ok(Self::Owner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),

            /* Context related players */
            "active player" => Ok(Self::ActivePlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "defending player" => Ok(Self::DefendingPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nearest opponent" => Ok(Self::NearestOpponent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting player" => Ok(Self::StartingPlayer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the monarch" => Ok(Self::TheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the player to your left" => Ok(Self::ToYourLeft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the player to your right" => Ok(Self::ToYourRight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "voter" => Ok(Self::Voter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
