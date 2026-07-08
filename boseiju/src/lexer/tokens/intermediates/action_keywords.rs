use crate::lexer::IntoToken;
use crate::lexer::tokens::tensed::Tensed;

/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionKeyword {
    Deals {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Get {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Put {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

pub type TensedActionKeyword = Tensed<ActionKeyword>;

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for ActionKeyword {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Deals { span } => *span,
            Self::Get { span } => *span,
            Self::Put { span } => *span,
        }
    }
}

impl IntoToken for TensedActionKeyword {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "deal" => Some(Tensed::base_form(ActionKeyword::Deals {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "deals" => Some(Tensed::third_person_singular_present(ActionKeyword::Deals {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "dealt" => Some(Tensed::simple_past(ActionKeyword::Deals {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "dealing" => Some(Tensed::present_participle(ActionKeyword::Deals {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "get" => Some(Tensed::base_form(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "gets" => Some(Tensed::third_person_singular_present(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "got" => Some(Tensed::simple_past(ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "put" => Some(Tensed::base_form(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "puts" => Some(Tensed::third_person_singular_present(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "putting" => Some(Tensed::present_participle(ActionKeyword::Put {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            _ => None,
        }
    }
}
