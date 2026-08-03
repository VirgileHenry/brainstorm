#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishWh {
    How {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    When {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whenever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Where {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whether {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Which {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whichever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Who {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Whose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishWh {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::How { span } => *span,
            Self::When { span } => *span,
            Self::Whenever { span } => *span,
            Self::Where { span } => *span,
            Self::Whether { span } => *span,
            Self::Which { span } => *span,
            Self::Whichever { span } => *span,
            Self::Who { span } => *span,
            Self::Whom { span } => *span,
            Self::Whose { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishWh {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "how" => Ok(Self::How {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "when" => Ok(Self::When {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whenever" => Ok(Self::Whenever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "where" => Ok(Self::Where {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whether" => Ok(Self::Whether {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "which" => Ok(Self::Which {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whichever" => Ok(Self::Whichever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "who" => Ok(Self::Who {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whom" => Ok(Self::Whom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "whose" => Ok(Self::Whose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
