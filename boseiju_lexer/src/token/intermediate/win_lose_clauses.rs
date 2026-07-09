#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinLoseClause {
    WinTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LoseTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGameIsADraw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for WinLoseClause {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::WinTheGame { span } => *span,
            Self::LoseTheGame { span } => *span,
            Self::TheGameIsADraw { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for WinLoseClause {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "win the game" | "wins the game" => Ok(Self::WinTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the game" | "loses the game" => Ok(Self::LoseTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the game is a draw" => Ok(Self::TheGameIsADraw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
