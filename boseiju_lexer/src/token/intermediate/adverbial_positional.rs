#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdverbialPositional {
    Back {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Bottom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Top {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AdverbialPositional {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Back { span } => *span,
            Self::Bottom { span } => *span,
            Self::Top { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AdverbialPositional {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "back" => Ok(Self::Back {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bottom" => Ok(Self::Bottom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "top" => Ok(Self::Top {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
