#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UncardSpecialTerm {
    HeightOfAtLeastOneFoot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Item {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OneTimeBoon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for UncardSpecialTerm {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::OneTimeBoon { span } => *span,
            Self::Item { span } => *span,
            Self::HeightOfAtLeastOneFoot { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for UncardSpecialTerm {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "height of at least one foot" => Ok(Self::HeightOfAtLeastOneFoot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "item" | "items" => Ok(Self::Item {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "one-time boon" => Ok(Self::OneTimeBoon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
