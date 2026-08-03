#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreatureGrouping {
    Outlaw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Party {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureGrouping {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Outlaw { span } => *span,
            Self::Party { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CreatureGrouping {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "outlaw" | "outlaws" => Ok(Self::Outlaw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "party" => Ok(Self::Party {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
