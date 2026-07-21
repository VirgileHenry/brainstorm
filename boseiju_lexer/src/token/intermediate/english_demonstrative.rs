#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishDemonstrative {
    That {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    These {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    This {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Those {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishDemonstrative {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::That { span } => *span,
            Self::These { span } => *span,
            Self::This { span } => *span,
            Self::Those { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishDemonstrative {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "that" => Ok(Self::That {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "these" => Ok(Self::These {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this" => Ok(Self::This {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "those" => Ok(Self::Those {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
