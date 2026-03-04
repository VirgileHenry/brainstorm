#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TapUntapCost {
    Tap {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Untap {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl TapUntapCost {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Tap { span } => *span,
            Self::Untap { span } => *span,
        }
    }
}

impl TapUntapCost {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "{t}" => Some(Self::Tap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "{q}" => Some(Self::Untap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
