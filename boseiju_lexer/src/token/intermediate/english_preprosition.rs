#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishPreprosition {
    Among {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    At {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Beyond {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    By {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    For {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    From {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    In {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Into {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Of {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    On {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Onto {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    To {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Under {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    With {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Without {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishPreprosition {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Among { span } => *span,
            Self::At { span } => *span,
            Self::Beyond { span } => *span,
            Self::By { span } => *span,
            Self::For { span } => *span,
            Self::From { span } => *span,
            Self::In { span } => *span,
            Self::Into { span } => *span,
            Self::Of { span } => *span,
            Self::On { span } => *span,
            Self::Onto { span } => *span,
            Self::To { span } => *span,
            Self::Under { span } => *span,
            Self::With { span } => *span,
            Self::Without { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishPreprosition {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "among" => Ok(Self::Among {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "at" => Ok(Self::At {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beyond" => Ok(Self::Beyond {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "by" => Ok(Self::By {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "for" => Ok(Self::For {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "from" => Ok(Self::From {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in" => Ok(Self::In {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "into" => Ok(Self::Into {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "of" => Ok(Self::Of {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "on" => Ok(Self::On {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "onto" => Ok(Self::Onto {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "to" => Ok(Self::To {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "under" => Ok(Self::Under {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "with" => Ok(Self::With {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "without" => Ok(Self::Without {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
