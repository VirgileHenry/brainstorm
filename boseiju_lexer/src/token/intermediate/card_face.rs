/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardFace {
    FaceDown {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FaceUp {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardFace {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::FaceDown { span } => *span,
            Self::FaceUp { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardFace {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "face down" | "face-down" => Ok(Self::FaceDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "face up" | "face-up" => Ok(Self::FaceUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
