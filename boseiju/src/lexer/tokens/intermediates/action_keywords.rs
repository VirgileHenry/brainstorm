/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionKeyword {
    Deals {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Get {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Put {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Reveal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl ActionKeyword {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Deals { span } => *span,
            Self::Get { span } => *span,
            Self::Put { span } => *span,
            Self::Reveal { span } => *span,
        }
    }
}

impl ActionKeyword {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "deal" | "deals" | "dealt" => Some(Self::Deals {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "get" | "gets" => Some(Self::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "put" | "puts" | "putting" => Some(Self::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reveal" | "reveals" => Some(Self::Reveal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
