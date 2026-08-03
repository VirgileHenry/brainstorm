#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AbilityProperty {
    ActivationCost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PointOfBushido {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AbilityProperty {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ActivationCost { span } => *span,
            Self::PointOfBushido { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AbilityProperty {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "activation cost" | "activation costs" => Ok(Self::ActivationCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "point of bushido" => Ok(Self::PointOfBushido {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
