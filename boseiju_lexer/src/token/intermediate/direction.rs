#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Direction {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Right {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /* Left is an ambiguous token */
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Direction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Direction { span } => *span,
            Self::Right { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Direction {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "direction" => Ok(Self::Direction {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "right" => Ok(Self::Right {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
