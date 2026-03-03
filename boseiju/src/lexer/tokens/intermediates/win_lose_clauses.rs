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
}

#[cfg(feature = "spanned_tree")]
impl WinLoseClause {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::WinTheGame { span } => *span,
            Self::LoseTheGame { span } => *span,
        }
    }
}

impl WinLoseClause {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "win the game" => Some(Self::WinTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the game" => Some(Self::LoseTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
