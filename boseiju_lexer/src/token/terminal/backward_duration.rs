/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackwardDuration {
    ThisGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /* Fixme: weird one */
    ThisTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for BackwardDuration {
    fn default() -> Self {
        Self::ThisGame {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for BackwardDuration {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ThisGame { span } => *span,
            Self::ThisTurn { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for BackwardDuration {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "this game" => Ok(Self::ThisGame {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            "this turn" | "so far this turn" => Ok(Self::ThisTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for BackwardDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThisGame { .. } => write!(f, "this game"),
            Self::ThisTurn { .. } => write!(f, "this turn"),
        }
    }
}
