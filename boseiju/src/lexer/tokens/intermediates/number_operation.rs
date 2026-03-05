#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOperation {
    Minus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Plus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl NumberOperation {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Minus { span } => *span,
            Self::Plus { span } => *span,
        }
    }
}

impl NumberOperation {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "minus" => Some(Self::Minus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plus" => Some(Self::Plus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
