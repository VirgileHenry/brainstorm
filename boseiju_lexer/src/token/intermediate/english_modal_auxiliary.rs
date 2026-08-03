#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishModalAuxiliary {
    Can {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cant {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Could {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Couldnt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    May {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Must {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Would {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishModalAuxiliary {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Can { span } => *span,
            Self::Cant { span } => *span,
            Self::Could { span } => *span,
            Self::Couldnt { span } => *span,
            Self::May { span } => *span,
            Self::Must { span } => *span,
            Self::Would { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishModalAuxiliary {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "can" => Ok(Self::Can {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "can't" => Ok(Self::Cant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "could" => Ok(Self::Could {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "couldn't" => Ok(Self::Couldnt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "may" => Ok(Self::May {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "must" => Ok(Self::Must {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "would" => Ok(Self::Would {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
