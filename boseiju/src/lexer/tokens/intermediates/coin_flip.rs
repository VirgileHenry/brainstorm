/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoinFlip {
    Coin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ComeUpHead {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ComeUpTails {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FlipYouWon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WinTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LoseTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StopFlipping {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CoinFlip {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Coin { span } => *span,
            Self::ComeUpHead { span } => *span,
            Self::ComeUpTails { span } => *span,
            Self::FlipYouWon { span } => *span,
            Self::WinTheFlip { span } => *span,
            Self::LoseTheFlip { span } => *span,
            Self::StopFlipping { span } => *span,
        }
    }
}

impl CoinFlip {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "coin" | "coins" => Some(Self::Coin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up heads" | "came up heads" | "comes up heads" => Some(Self::ComeUpHead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up tails" | "came up tails" | "comes up tails" => Some(Self::ComeUpTails {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip you won" => Some(Self::FlipYouWon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win the flip" | "wins a coin flip" => Some(Self::WinTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the flip" => Some(Self::LoseTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stop flipping" => Some(Self::StopFlipping {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
