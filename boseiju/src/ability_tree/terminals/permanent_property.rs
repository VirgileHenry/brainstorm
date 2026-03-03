use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermanentProperty {
    Power {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tougness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ConvertedManaCost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PermanentProperty {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Power { span } => *span,
            Self::Tougness { span } => *span,
            Self::ConvertedManaCost { span } => *span,
        }
    }
}

impl std::fmt::Display for PermanentProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentProperty::Power { .. } => write!(f, "power"),
            PermanentProperty::Tougness { .. } => write!(f, "touhness"),
            PermanentProperty::ConvertedManaCost { .. } => write!(f, "converted mana cost"),
        }
    }
}

impl IntoToken for PermanentProperty {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "power" => Some(PermanentProperty::Power {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toughness" => Some(PermanentProperty::Tougness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana cost" | "mana value" => Some(PermanentProperty::ConvertedManaCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
