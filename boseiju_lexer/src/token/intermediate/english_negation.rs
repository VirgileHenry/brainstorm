#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishNegation {
    No {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NoLonger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Not {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishNegation {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::No { span } => *span,
            Self::NoLonger { span } => *span,
            Self::Not { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishNegation {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "no" => Ok(Self::No {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "no longer" => Ok(Self::NoLonger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "not" => Ok(Self::Not {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
