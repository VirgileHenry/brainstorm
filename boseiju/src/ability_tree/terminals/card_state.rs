use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardState {
    Attached {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
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
    Exiled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Revealed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Sacrificed {
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
impl CardState {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attached { span } => *span,
            Self::Attacking { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span } => *span,
            Self::Enchanted { span } => *span,
            Self::Equipped { span } => *span,
            Self::Exiled { span } => *span,
            Self::Revealed { span } => *span,
            Self::Sacrificed { span } => *span,
            Self::Tapped { span } => *span,
            Self::Untapped { span } => *span,
        }
    }
}

impl std::fmt::Display for CardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardState::Attached { .. } => write!(f, "attached"),
            CardState::Attacking { .. } => write!(f, "attacking"),
            CardState::Blocking { .. } => write!(f, "blocking"),
            CardState::Blocked { .. } => write!(f, "blocked"),
            CardState::Enchanted { .. } => write!(f, "enchanted"),
            CardState::Equipped { .. } => write!(f, "equipped"),
            CardState::Exiled { .. } => write!(f, "exiled"),
            CardState::Revealed { .. } => write!(f, "revealed"),
            CardState::Sacrificed { .. } => write!(f, "sacrificed"),
            CardState::Tapped { .. } => write!(f, "tapped"),
            CardState::Untapped { .. } => write!(f, "untapped"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for CardState {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "attached" => Some(CardState::Attached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacking" => Some(CardState::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocking" => Some(CardState::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocked" => Some(CardState::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted" => Some(CardState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "equipped" => Some(CardState::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exiled" => Some(CardState::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "revealed" => Some(CardState::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sacrificed" => Some(CardState::Sacrificed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Some(CardState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "untapped" => Some(CardState::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
