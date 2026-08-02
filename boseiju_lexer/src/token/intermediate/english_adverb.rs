#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishAdverb {
    Continuously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Differently {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Perpetually {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishAdverb {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Continuously { span } => *span,
            Self::Differently { span } => *span,
            Self::Perpetually { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishAdverb {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "continuously" => Ok(Self::Continuously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "differently" => Ok(Self::Differently {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "perpetually" => Ok(Self::Perpetually {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
