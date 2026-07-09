/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedDungeon {
    TombOfAnnihilation {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedDungeon {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::TombOfAnnihilation { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedDungeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedDungeon::TombOfAnnihilation { .. } => write!(f, "tomb of annihilation"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedDungeon {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "tomb of annihilation" => Ok(NamedDungeon::TombOfAnnihilation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
