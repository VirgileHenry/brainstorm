#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Paw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Phyrexian {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Symbol {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Paw { span } => *span,
            Self::Phyrexian { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Symbol {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "{p}" => Ok(Self::Paw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "{h}" => Ok(Self::Phyrexian {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
