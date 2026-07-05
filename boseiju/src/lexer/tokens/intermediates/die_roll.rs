/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DieRoll {
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
    D10 {
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
    Natural20 {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NaturalResult {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlanarDie {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Result {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl DieRoll {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::D4 { span } => *span,
            Self::D6 { span } => *span,
            Self::D8 { span } => *span,
            Self::D10 { span } => *span,
            Self::D12 { span } => *span,
            Self::D20 { span } => *span,
            Self::DieRange { span, .. } => *span,
            Self::Natural20 { span, .. } => *span,
            Self::NaturalResult { span } => *span,
            Self::PlanarDie { span } => *span,
            Self::Result { span } => *span,
        }
    }
}

impl DieRoll {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "d4" | "four-sided die" | "four-sided dice" => Some(Self::D4 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d6" | "six-sided die" | "six-sided dice" => Some(Self::D6 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d8" | "eight-sided die" | "eight-sided dice" => Some(Self::D8 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d10" | "ten-sided die" | "ten-sided dice" => Some(Self::D10 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d12" | "twelve-sided die" | "twelve-sided dice" => Some(Self::D12 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d20" | "twenty-sided die" | "twenty-sided dice" => Some(Self::D20 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "natural 20" => Some(Self::NaturalResult {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "natural result" => Some(Self::NaturalResult {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planar die" | "planar dice" => Some(Self::PlanarDie {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "result" | "results" => Some(Self::Result {
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
