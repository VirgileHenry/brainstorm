#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayNight {
    Day {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Night {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl DayNight {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Day { span } => *span,
            Self::Night { span } => *span,
        }
    }
}

impl DayNight {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "day" => Some(Self::Day {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "night" => Some(Self::Night {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
