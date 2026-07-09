#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpecialCost {
    Paw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpecialCost {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Paw { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for SpecialCost {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "{p}" => Ok(Self::Paw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
