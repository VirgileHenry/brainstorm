#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishAdjective {
    Common {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unchanged {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishAdjective {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Common { span } => *span,
            Self::Unchanged { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishAdjective {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "common" => Ok(Self::Common {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unchanged" => Ok(Self::Unchanged {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
