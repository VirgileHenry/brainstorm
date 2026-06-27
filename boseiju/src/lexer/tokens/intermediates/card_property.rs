use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardProperty {
    BasePower {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BasePowerAndToughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BaseToughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Commander {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Historic {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Level {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ManaValue {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Multicolored {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Name {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Power {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Toughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CardProperty {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::BasePower { span } => *span,
            Self::BasePowerAndToughness { span } => *span,
            Self::BaseToughness { span } => *span,
            Self::Commander { span } => *span,
            Self::Historic { span } => *span,
            Self::Level { span } => *span,
            Self::ManaValue { span } => *span,
            Self::Multicolored { span } => *span,
            Self::Name { span } => *span,
            Self::Power { span } => *span,
            Self::Toughness { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for CardProperty {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "base power" => Some(CardProperty::BasePower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base power and toughness" => Some(CardProperty::BasePowerAndToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base toughness" => Some(CardProperty::BaseToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "commander" => Some(CardProperty::Commander {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "historic" => Some(CardProperty::Historic {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "level" => Some(CardProperty::Level {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana cost" | "mana value" => Some(CardProperty::ManaValue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "multicolored" => Some(CardProperty::Multicolored {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "name" => Some(CardProperty::Name {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "power" => Some(CardProperty::Power {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toughness" => Some(CardProperty::Toughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
