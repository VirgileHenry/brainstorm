use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermanentState {
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enchanted {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Equipped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Untapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PermanentState {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacking { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span } => *span,
            Self::Enchanted { span } => *span,
            Self::Equipped { span } => *span,
            Self::Tapped { span } => *span,
            Self::Untapped { span } => *span,
        }
    }
}

impl std::fmt::Display for PermanentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentState::Attacking { .. } => write!(f, "attacking"),
            PermanentState::Blocking { .. } => write!(f, "blocking"),
            PermanentState::Blocked { .. } => write!(f, "blocked"),
            PermanentState::Enchanted { .. } => write!(f, "enchanted"),
            PermanentState::Equipped { .. } => write!(f, "equipped"),
            PermanentState::Tapped { .. } => write!(f, "tapped"),
            PermanentState::Untapped { .. } => write!(f, "untapped"),
        }
    }
}

impl IntoToken for PermanentState {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "attacking" => Some(PermanentState::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocking" => Some(PermanentState::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocked" => Some(PermanentState::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted" => Some(PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "equipped" => Some(PermanentState::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Some(PermanentState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "untapped" => Some(PermanentState::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
