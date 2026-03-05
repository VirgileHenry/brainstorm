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
    Color {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ManaValue {
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
    Tougness {
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
            Self::Color { span } => *span,
            Self::ManaValue { span } => *span,
            Self::Name { span } => *span,
            Self::Power { span } => *span,
            Self::Tougness { span } => *span,
        }
    }
}

impl std::fmt::Display for CardProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardProperty::BasePower { .. } => write!(f, "converted mana cost"),
            CardProperty::BasePowerAndToughness { .. } => write!(f, "converted mana cost"),
            CardProperty::BaseToughness { .. } => write!(f, "converted mana cost"),
            CardProperty::Color { .. } => write!(f, "converted mana cost"),
            CardProperty::ManaValue { .. } => write!(f, "converted mana cost"),
            CardProperty::Name { .. } => write!(f, "converted mana cost"),
            CardProperty::Power { .. } => write!(f, "power"),
            CardProperty::Tougness { .. } => write!(f, "touhness"),
        }
    }
}

impl IntoToken for CardProperty {
    #[cfg(feature = "lexer")]
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
            "color" | "colors" => Some(CardProperty::Color {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana cost" | "mana value" => Some(CardProperty::ManaValue {
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
            "toughness" => Some(CardProperty::Tougness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
