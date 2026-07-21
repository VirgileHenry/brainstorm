#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishDeterminer {
    Another {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Different {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Every {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    New {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Other {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Same {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishDeterminer {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Another { span } => *span,
            Self::Any { span } => *span,
            Self::Different { span } => *span,
            Self::Every { span } => *span,
            Self::New { span } => *span,
            Self::Other { span } => *span,
            Self::Same { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishDeterminer {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "another" => Ok(Self::Another {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any" => Ok(Self::Any {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "different" => Ok(Self::Different {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "every" => Ok(Self::Every {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "new" => Ok(Self::New {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "other" => Ok(Self::Other {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "same" => Ok(Self::Same {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
