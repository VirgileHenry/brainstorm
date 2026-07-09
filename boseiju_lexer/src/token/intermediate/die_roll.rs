/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DieRoll {
    D4 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    D6 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    D8 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    D10 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    D12 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    D20 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DieRange {
        start: usize,
        end: usize,
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Natural20 {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NaturalResult {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlanarDie {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Result {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DieRoll {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for DieRoll {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "d4" | "four-sided die" | "four-sided dice" => Ok(Self::D4 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d6" | "six-sided die" | "six-sided dice" => Ok(Self::D6 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d8" | "eight-sided die" | "eight-sided dice" => Ok(Self::D8 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d10" | "ten-sided die" | "ten-sided dice" => Ok(Self::D10 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d12" | "twelve-sided die" | "twelve-sided dice" => Ok(Self::D12 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "d20" | "twenty-sided die" | "twenty-sided dice" => Ok(Self::D20 {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "natural 20" => Ok(Self::NaturalResult {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "natural result" => Ok(Self::NaturalResult {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planar die" | "planar dice" => Ok(Self::PlanarDie {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "result" | "results" => Ok(Self::Result {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            other => {
                let split: Vec<&str> = other.split('—').collect();
                match split.as_slice() {
                    &[start, end] => match (start.parse::<usize>(), end.parse::<usize>()) {
                        (Ok(start), Ok(end)) => Ok(Self::DieRange {
                            start,
                            end,
                            #[cfg(feature = "spanned_tree")]
                            span: span.into(),
                        }),
                        _ => Err(()),
                    },
                    _ => Err(()),
                }
            }
        }
    }
}
