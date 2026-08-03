#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TapUntapCost {
    Tap {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Untap {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TapUntapCost {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Tap { span } => *span,
            Self::Untap { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for TapUntapCost {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "{t}" => Ok(Self::Tap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "{q}" => Ok(Self::Untap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
