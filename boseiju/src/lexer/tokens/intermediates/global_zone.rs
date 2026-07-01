#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalZone {
    Anywhere {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CommandZone {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OutsideTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheBattlefield {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl GlobalZone {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Anywhere { span } => *span,
            Self::CommandZone { span } => *span,
            Self::OutsideTheGame { span } => *span,
            Self::TheBattlefield { span } => *span,
        }
    }
}

impl GlobalZone {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "anywhere" => Some(Self::Anywhere {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "command zone" => Some(Self::CommandZone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "outside the game" => Some(Self::OutsideTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the battlefield" => Some(Self::TheBattlefield {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
