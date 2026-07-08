#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinLoseClause {
    WinTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LoseTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGameIsADraw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for WinLoseClause {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::WinTheGame { span } => *span,
            Self::LoseTheGame { span } => *span,
            Self::TheGameIsADraw { span } => *span,
        }
    }
}

impl WinLoseClause {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "win the game" | "wins the game" => Some(Self::WinTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the game" | "loses the game" => Some(Self::LoseTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the game is a draw" => Some(Self::TheGameIsADraw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
