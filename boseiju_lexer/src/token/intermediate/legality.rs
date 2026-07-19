#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Legality {
    Illegal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Legal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Legality {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Illegal { span } => *span,
            Self::Legal { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Legality {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "illegal" => Ok(Self::Illegal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legal" => Ok(Self::Legal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
