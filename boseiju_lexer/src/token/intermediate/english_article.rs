#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishArticle {
    A {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    An {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    The {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishArticle {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::A { span } => *span,
            Self::An { span } => *span,
            Self::The { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishArticle {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "a" => Ok(Self::A {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "an" => Ok(Self::An {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the" => Ok(Self::The {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
