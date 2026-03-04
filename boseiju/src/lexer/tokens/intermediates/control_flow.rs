#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlFlow {
    NewLine {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Comma {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Dot {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Colons {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LongDash {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bullet {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl ControlFlow {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::NewLine { span } => *span,
            Self::Comma { span } => *span,
            Self::Dot { span } => *span,
            Self::Colons { span } => *span,
            Self::LongDash { span } => *span,
            Self::Bullet { span } => *span,
        }
    }
}

impl ControlFlow {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "\n" => Some(ControlFlow::NewLine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "," => Some(ControlFlow::Comma {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "." => Some(ControlFlow::Dot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            ":" => Some(ControlFlow::Colons {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "—" => Some(ControlFlow::LongDash {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "•" => Some(ControlFlow::Bullet {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
