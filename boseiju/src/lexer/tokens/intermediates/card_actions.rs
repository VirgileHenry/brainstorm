#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    Blocks {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Dies {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enters {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Leave {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CardActions {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Blocks { span } => *span,
            Self::Dies { span } => *span,
            Self::Enters { span } => *span,
            Self::Fight { span } => *span,
            Self::Leave { span } => *span,
        }
    }
}

impl CardActions {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "block" | "blocks" => Some(Self::Blocks {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "die" | "dies" => Some(Self::Dies {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enter" | "enters" => Some(Self::Enters {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fight" | "fights" => Some(Self::Fight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "leave" | "leaves" => Some(Self::Leave {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
