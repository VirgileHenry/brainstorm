#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdverbialAdditive {
    Additional {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AdditionalTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Also {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Likewise {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AdverbialAdditive {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Additional { span } => *span,
            Self::AdditionalTime { span } => *span,
            Self::Also { span } => *span,
            Self::Likewise { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AdverbialAdditive {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "additional" => Ok(Self::Additional {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "additional time" => Ok(Self::AdditionalTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "also" => Ok(Self::Also {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "likewise" => Ok(Self::Likewise {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
