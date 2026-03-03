#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PowerToughnessModElements {
    Plus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Minus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bar {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PowerToughnessModElements {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Plus { span } => *span,
            Self::Minus { span } => *span,
            Self::Bar { span } => *span,
        }
    }
}

impl PowerToughnessModElements {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "+" => Some(Self::Plus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "-" => Some(Self::Minus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "/" => Some(Self::Bar {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
