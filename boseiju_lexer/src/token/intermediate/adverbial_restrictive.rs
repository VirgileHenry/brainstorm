#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdverbialRestrictive {
    Alone {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Only {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Single {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AdverbialRestrictive {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Alone { span } => *span,
            Self::Only { span } => *span,
            Self::Single { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AdverbialRestrictive {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "alone" => Ok(Self::Alone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "only" => Ok(Self::Only {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "single" => Ok(Self::Single {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
