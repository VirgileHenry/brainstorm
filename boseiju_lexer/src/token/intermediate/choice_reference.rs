#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChoiceReference {
    Color {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Direction {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HasntBeenChosen {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Quality {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ChoiceReference {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Color { span } => *span,
            Self::Direction { span } => *span,
            Self::HasntBeenChosen { span } => *span,
            Self::Quality { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ChoiceReference {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "chosen color" => Ok(Self::Color {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chosen direction" => Ok(Self::Direction {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hasn't been chosen" => Ok(Self::HasntBeenChosen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quality" => Ok(Self::Quality {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
