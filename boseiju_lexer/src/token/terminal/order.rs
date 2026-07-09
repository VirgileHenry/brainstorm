/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Order {
    RandomOrder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChosenOrder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Order {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::RandomOrder { span } => *span,
            Self::ChosenOrder { span } => *span,
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::RandomOrder { .. } => write!(f, "a random order"),
            Order::ChosenOrder { .. } => write!(f, "any order"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Order {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "a random order" => Ok(Order::RandomOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "any order" => Ok(Order::ChosenOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
