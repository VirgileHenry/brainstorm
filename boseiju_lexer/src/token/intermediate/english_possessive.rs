#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishPossessive {
    HisHers {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Their {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Yours {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishPossessive {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::HisHers { span } => *span,
            Self::Their { span } => *span,
            Self::Yours { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishPossessive {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "his" | "her" => Ok(Self::HisHers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "their" => Ok(Self::Their {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yours" => Ok(Self::Yours {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
