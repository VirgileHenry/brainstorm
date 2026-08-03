/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MiscObjectName {
    Door {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for MiscObjectName {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Door { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for MiscObjectName {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "door" | "doors" => Ok(Self::Door {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
