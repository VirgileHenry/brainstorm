/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Order {
    Random {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Chosen {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for Order {
    fn default() -> Self {
        Self::Random {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Order {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Random { span } => *span,
            Self::Chosen { span } => *span,
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Random { .. } => write!(f, "a random order"),
            Order::Chosen { .. } => write!(f, "any order"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Order {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "a random order" => Ok(Order::Random {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any order" => Ok(Order::Chosen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
