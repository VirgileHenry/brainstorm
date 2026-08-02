/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Choice {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mode {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WorthOfModes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Mode {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Choice { span } => *span,
            Self::Mode { span } => *span,
            Self::WorthOfModes { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Mode {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "choice" | "choices" => Ok(Self::Choice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mode" => Ok(Self::Mode {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "worth of modes" => Ok(Self::WorthOfModes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
