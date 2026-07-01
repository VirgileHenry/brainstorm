use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardState {
    Able {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Attached {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bargained {
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
    Goaded {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Modified {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Monstrous {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Paired {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Remains {
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
    Saddled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGiftWasPromised {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unblocked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Untapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Warped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl CardState {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Able {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Attached {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Bargained {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Goaded {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Modified {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Monstrous {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Paired {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Remains {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Sacrificed {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Saddled {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::TheGiftWasPromised {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Unblocked {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Warped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }
}

#[cfg(feature = "spanned_tree")]
impl CardState {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Able { span } => *span,
            Self::Attached { span } => *span,
            Self::Attacking { span, .. } => *span,
            Self::Bargained { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span, .. } => *span,
            Self::Enchanted { span } => *span,
            Self::Equipped { span } => *span,
            Self::Exiled { span, .. } => *span,
            Self::Goaded { span, .. } => *span,
            Self::Modified { span, .. } => *span,
            Self::Monstrous { span, .. } => *span,
            Self::Paired { span, .. } => *span,
            Self::Remains { span } => *span,
            Self::Revealed { span } => *span,
            Self::Sacrificed { span } => *span,
            Self::Saddled { span } => *span,
            Self::Tapped { span } => *span,
            Self::TheGiftWasPromised { span } => *span,
            Self::Unblocked { span } => *span,
            Self::Untapped { span } => *span,
            Self::Warped { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for CardState {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "able" => Some(CardState::Able {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attached" => Some(CardState::Attached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacking" => Some(CardState::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bargained" => Some(CardState::Bargained {
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
            "goaded" => Some(CardState::Goaded {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "modified" => Some(CardState::Modified {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "monstrous" => Some(CardState::Monstrous {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "paired" => Some(CardState::Paired {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "remain" | "remains" => Some(CardState::Remains {
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
            "saddled" | "saddles" => Some(CardState::Saddled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Some(CardState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the gift was promised" => Some(CardState::TheGiftWasPromised {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unblocked" => Some(CardState::Unblocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "untapped" => Some(CardState::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "warped" => Some(CardState::Warped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardState {
    fn dummy_init() -> Self {
        Self::Attacking {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
