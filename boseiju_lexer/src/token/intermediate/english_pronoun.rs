#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnglishPronoun {
    It {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Them {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    They {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnglishPronoun {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::It { span } => *span,
            Self::Them { span } => *span,
            Self::They { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnglishPronoun {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "it" => Ok(Self::It {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "them" => Ok(Self::Them {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "they" => Ok(Self::They {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
