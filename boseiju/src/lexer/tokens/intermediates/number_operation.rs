#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOperation {
    Above {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Below {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Between {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Difference {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Divide {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Even {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Excess {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Higher {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Highest {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Increased {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Lower {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Lowest {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Match {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MaximumOf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Minus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Odd {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Plus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PowerX {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Reduce {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundDown {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Smaller {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Substract {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for NumberOperation {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Above { span } => *span,
            Self::Below { span } => *span,
            Self::Between { span } => *span,
            Self::Difference { span } => *span,
            Self::Divide { span } => *span,
            Self::Even { span } => *span,
            Self::Excess { span } => *span,
            Self::Higher { span } => *span,
            Self::Highest { span } => *span,
            Self::Increased { span } => *span,
            Self::Lower { span } => *span,
            Self::Lowest { span } => *span,
            Self::Match { span } => *span,
            Self::MaximumOf { span } => *span,
            Self::Minus { span } => *span,
            Self::Odd { span } => *span,
            Self::Plus { span } => *span,
            Self::PowerX { span } => *span,
            Self::Reduce { span } => *span,
            Self::RoundDown { span } => *span,
            Self::RoundUp { span } => *span,
            Self::Smaller { span } => *span,
            Self::Substract { span } => *span,
        }
    }
}

impl NumberOperation {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "above" => Some(Self::Above {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "below" => Some(Self::Below {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "between" => Some(Self::Between {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "difference" => Some(Self::Difference {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divide" | "divides" => Some(Self::Divide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "even" => Some(Self::Even {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "excess" => Some(Self::Excess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "higher" => Some(Self::Higher {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "highest" => Some(Self::Highest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "increased" => Some(Self::Increased {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lower" => Some(Self::Lower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lowest" => Some(Self::Lowest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "match" => Some(Self::Match {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "maximum of" => Some(Self::MaximumOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "minus" => Some(Self::Minus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "odd" => Some(Self::Odd {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plus" => Some(Self::Plus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ˣ" => Some(Self::PowerX {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reduce" | "reduced" | "reduces" => Some(Self::Reduce {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "round down" => Some(Self::RoundDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "round up" => Some(Self::RoundUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "smaller" => Some(Self::Smaller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "subtract" => Some(Self::Substract {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
