/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoinFlip {
    Coin {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ComeUpHead {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ComeUpTails {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FlipYouWon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WinTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LoseTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StopFlipping {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CoinFlip {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CoinFlip {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "coin" | "coins" => Ok(Self::Coin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up heads" | "came up heads" | "comes up heads" => Ok(Self::ComeUpHead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up tails" | "came up tails" | "comes up tails" => Ok(Self::ComeUpTails {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip you won" => Ok(Self::FlipYouWon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win the flip" | "wins a coin flip" => Ok(Self::WinTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the flip" => Ok(Self::LoseTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stop flipping" => Ok(Self::StopFlipping {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
