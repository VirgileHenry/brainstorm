#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayNight {
    Day {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Night {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DayNight {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Day { span } => *span,
            Self::Night { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for DayNight {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "day" => Ok(Self::Day {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "night" => Ok(Self::Night {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
