use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellProperty {
    Countered {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Kicked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Resolved {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl SpellProperty {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Countered { span } => *span,
            Self::Kicked { span } => *span,
            Self::Resolved { span } => *span,
        }
    }
}

impl std::fmt::Display for SpellProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellProperty::Countered { .. } => write!(f, "countered"),
            SpellProperty::Kicked { .. } => write!(f, "kicked"),
            SpellProperty::Resolved { .. } => write!(f, "kicked"),
        }
    }
}

impl IntoToken for SpellProperty {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "countered" => Some(SpellProperty::Countered {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kicked" => Some(SpellProperty::Kicked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "resolved" => Some(SpellProperty::Resolved {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
