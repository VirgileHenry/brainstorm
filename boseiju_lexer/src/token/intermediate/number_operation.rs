#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOperation {
    Above {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BarSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Below {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Between {
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
    Higher {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Highest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Increased {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Lower {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Lowest {
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
    Smaller {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Substract {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NumberOperation {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Above { span } => *span,
            Self::BarSymbol { span } => *span,
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
            Self::Smaller { span } => *span,
            Self::Substract { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NumberOperation {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "above" => Ok(Self::Above {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "/" => Ok(Self::BarSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "below" => Ok(Self::Below {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "between" => Ok(Self::Between {
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
            "higher" => Ok(Self::Higher {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "highest" => Ok(Self::Highest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "increased" => Ok(Self::Increased {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lower" => Ok(Self::Lower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lowest" => Ok(Self::Lowest {
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
            "smaller" => Ok(Self::Smaller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "subtract" => Ok(Self::Substract {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
