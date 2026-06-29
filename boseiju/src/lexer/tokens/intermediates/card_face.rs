/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardFace {
    FaceDown {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FaceUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CardFace {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::FaceDown { span } => *span,
            Self::FaceUp { span } => *span,
        }
    }
}

impl CardFace {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "face down" | "face-down" => Some(Self::FaceDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "face up" | "face-up" => Some(Self::FaceUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
