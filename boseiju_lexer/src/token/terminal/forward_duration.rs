/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ForwardDuration {
    ForAsLongAsItsExiled {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Forever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UntilEndOfTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UntilEndOfYourNextTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for ForwardDuration {
    fn default() -> Self {
        Self::Forever {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl boseiju_span::Spanned for ForwardDuration {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ForAsLongAsItsExiled { span } => *span,
            Self::Forever { span } => *span,
            Self::UntilEndOfTurn { span } => *span,
            Self::UntilEndOfYourNextTurn { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ForwardDuration {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "until end of turn" => Ok(Self::UntilEndOfTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            "until the end of your next turn" => Ok(Self::UntilEndOfYourNextTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            "for the rest of the game" => Ok(Self::Forever {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ForwardDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForAsLongAsItsExiled { .. } => write!(f, "for as long as it remains exiled"),
            Self::Forever { .. } => write!(f, "forever"),
            Self::UntilEndOfTurn { .. } => write!(f, "until end of turn"),
            Self::UntilEndOfYourNextTurn { .. } => write!(f, "until the end of your next turn"),
        }
    }
}
