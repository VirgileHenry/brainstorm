/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Die {
    D4 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    D6 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    D8 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    D12 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    D20 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DieRange {
        start: usize,
        end: usize,
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Result {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl Die {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::D4 { span } => *span,
            Self::D6 { span } => *span,
            Self::D8 { span } => *span,
            Self::D12 { span } => *span,
            Self::D20 { span } => *span,
            Self::DieRange { span, .. } => *span,
            Self::Result { span } => *span,
        }
    }
}

impl Die {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "d4" => Some(Self::D4 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d6" => Some(Self::D6 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d8" => Some(Self::D8 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d12" => Some(Self::D12 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d20" => Some(Self::D20 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "result" => Some(Self::Result {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            other => {
                let split: Vec<&str> = other.split('—').collect();
                match split.as_slice() {
                    &[start, end] => match (start.parse::<usize>(), end.parse::<usize>()) {
                        (Ok(start), Ok(end)) => Some(Self::DieRange {
                            start,
                            end,
                            #[cfg(feature = "spanned_tree")]
                            span: span.into(),
                        }),
                        _ => None,
                    },
                    _ => None,
                }
            }
        }
    }
}
