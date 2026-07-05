#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChoiceReference {
    Color {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Direction {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HasntBeenChosen {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Quality {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl ChoiceReference {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Color { span } => *span,
            Self::Direction { span } => *span,
            Self::HasntBeenChosen { span } => *span,
            Self::Quality { span } => *span,
        }
    }
}

impl ChoiceReference {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "chosen color" => Some(Self::Color {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chosen direction" => Some(Self::Direction {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hasn't been chosen" => Some(Self::HasntBeenChosen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quality" => Some(Self::Quality {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
