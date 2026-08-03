#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishComparison {
    Above {
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
    Equal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exactly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fewer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fewest {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Greater {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Greatest {
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
    Least {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Less {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Lesser {
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
    More {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Most {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Smaller {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Than {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Tied {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishComparison {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Above { span } => *span,
            Self::Below { span } => *span,
            Self::Between { span } => *span,
            Self::Equal { span } => *span,
            Self::Exactly { span } => *span,
            Self::Fewer { span } => *span,
            Self::Fewest { span } => *span,
            Self::Greater { span } => *span,
            Self::Greatest { span } => *span,
            Self::Higher { span } => *span,
            Self::Highest { span } => *span,
            Self::Least { span } => *span,
            Self::Less { span } => *span,
            Self::Lesser { span } => *span,
            Self::Lower { span } => *span,
            Self::Lowest { span } => *span,
            Self::More { span } => *span,
            Self::Most { span } => *span,
            Self::Smaller { span } => *span,
            Self::Than { span } => *span,
            Self::Tied { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishComparison {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "above" => Ok(Self::Above {
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
            "equal" => Ok(Self::Equal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exactly" => Ok(Self::Exactly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fewer" => Ok(Self::Fewer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fewest" => Ok(Self::Fewest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "greater" => Ok(Self::Greater {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "greatest" => Ok(Self::Greatest {
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
            "least" => Ok(Self::Least {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "less" => Ok(Self::Less {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lesser" => Ok(Self::Lesser {
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
            "more" => Ok(Self::More {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "most" => Ok(Self::Most {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "smaller" => Ok(Self::Smaller {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "than" => Ok(Self::Than {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tie" | "tied" => Ok(Self::Tied {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
