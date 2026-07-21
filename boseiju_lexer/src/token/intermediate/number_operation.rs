#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOperation {
    BarSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CombinationOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Difference {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Divide {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Even {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Excess {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Half {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Increased {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Match {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MaximumOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Minus {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MinusSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Odd {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Plus {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlusSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PowerX {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Reduce {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundDown {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundedDown {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundedUp {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundUp {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Substract {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Times {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Total {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Twice {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NumberOperation {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::BarSymbol { span } => *span,
            Self::CombinationOf { span } => *span,
            Self::Difference { span } => *span,
            Self::Divide { span } => *span,
            Self::Even { span } => *span,
            Self::Excess { span } => *span,
            Self::Half { span } => *span,
            Self::Increased { span } => *span,
            Self::Match { span } => *span,
            Self::MaximumOf { span } => *span,
            Self::Minus { span } => *span,
            Self::MinusSymbol { span } => *span,
            Self::Odd { span } => *span,
            Self::Plus { span } => *span,
            Self::PlusSymbol { span } => *span,
            Self::PowerX { span } => *span,
            Self::Reduce { span } => *span,
            Self::RoundDown { span } => *span,
            Self::RoundedDown { span } => *span,
            Self::RoundedUp { span } => *span,
            Self::RoundUp { span } => *span,
            Self::Substract { span } => *span,
            Self::Times { span } => *span,
            Self::Total { span } => *span,
            Self::Twice { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NumberOperation {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "/" => Ok(Self::BarSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combination of" => Ok(Self::CombinationOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "difference" => Ok(Self::Difference {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "divide" | "divides" => Ok(Self::Divide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "even" => Ok(Self::Even {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "excess" => Ok(Self::Excess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "half" => Ok(Self::Half {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "increased" => Ok(Self::Increased {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "match" => Ok(Self::Match {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "maximum of" => Ok(Self::MaximumOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "minus" => Ok(Self::Minus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "-" => Ok(Self::MinusSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "odd" => Ok(Self::Odd {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plus" => Ok(Self::Plus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "+" => Ok(Self::PlusSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ˣ" => Ok(Self::PowerX {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reduce" | "reduced" | "reduces" => Ok(Self::Reduce {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "round down" => Ok(Self::RoundDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded down" => Ok(Self::RoundedDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded up" => Ok(Self::RoundedUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "round up" => Ok(Self::RoundUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "subtract" => Ok(Self::Substract {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "times" => Ok(Self::Times {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "total" => Ok(Self::Total {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "twice" => Ok(Self::Twice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
