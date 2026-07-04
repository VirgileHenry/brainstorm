use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardState {
    Able {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Amassed {
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
    Convoked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DeclaredAs {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Devoured {
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
    Escaped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EvidenceWasCollected {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Exiled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Exploited {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Foretold {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Goaded {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LockedDoor {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LookedAt {
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
    Mutated {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Paired {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Plotted {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Prepared {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Remains {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Renowned {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Revealed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RingBearer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Saddled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Suspected {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Targeted {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGiftWasPromised {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGiftWasntPromised {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Transformed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unattached {
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

#[cfg(feature = "spanned_tree")]
impl CardState {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Able { span } => *span,
            Self::Amassed { span } => *span,
            Self::Attached { span } => *span,
            Self::Attacking { span } => *span,
            Self::Bargained { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span } => *span,
            Self::Convoked { span } => *span,
            Self::DeclaredAs { span } => *span,
            Self::Devoured { span } => *span,
            Self::Enchanted { span } => *span,
            Self::Equipped { span } => *span,
            Self::Escaped { span } => *span,
            Self::EvidenceWasCollected { span } => *span,
            Self::Exiled { span } => *span,
            Self::Exploited { span } => *span,
            Self::Foretold { span } => *span,
            Self::Goaded { span } => *span,
            Self::LockedDoor { span } => *span,
            Self::LookedAt { span } => *span,
            Self::Modified { span } => *span,
            Self::Monstrous { span } => *span,
            Self::Mutated { span } => *span,
            Self::Paired { span } => *span,
            Self::Plotted { span } => *span,
            Self::Prepared { span } => *span,
            Self::Remains { span } => *span,
            Self::Renowned { span } => *span,
            Self::RingBearer { span } => *span,
            Self::Revealed { span } => *span,
            Self::Saddled { span } => *span,
            Self::Suspected { span } => *span,
            Self::Tapped { span } => *span,
            Self::Targeted { span } => *span,
            Self::TheGiftWasPromised { span } => *span,
            Self::TheGiftWasntPromised { span } => *span,
            Self::Transformed { span } => *span,
            Self::Unattached { span } => *span,
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
            "amassed" => Some(CardState::Amassed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attached" => Some(CardState::Attached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacking" | "attacker" => Some(CardState::Attacking {
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
            "declared as" => Some(CardState::DeclaredAs {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "convoked" => Some(CardState::Convoked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "devoured" => Some(CardState::Devoured {
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
            "escaped" => Some(CardState::Escaped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evidence was collected" => Some(CardState::EvidenceWasCollected {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exiled" => Some(CardState::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exploited" => Some(CardState::Exploited {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "foretold" => Some(CardState::Foretold {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "goaded" => Some(CardState::Goaded {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "locked door" => Some(CardState::LockedDoor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "looked at" => Some(CardState::LookedAt {
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
            "mutated" => Some(CardState::Mutated {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "paired" => Some(CardState::Paired {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plotted" => Some(CardState::Plotted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prepared" => Some(CardState::Prepared {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "remain" | "remains" => Some(CardState::Remains {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "renowned" => Some(CardState::Renowned {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ring-bearer" | "ring-bearers" => Some(CardState::RingBearer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "revealed" => Some(CardState::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "saddled" | "saddles" => Some(CardState::Saddled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "suspected" => Some(CardState::Suspected {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Some(CardState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "targeted" => Some(CardState::Targeted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the gift was promised" => Some(CardState::TheGiftWasPromised {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the gift wasn't promised" => Some(CardState::TheGiftWasntPromised {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "transformed" => Some(CardState::Transformed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unattached" => Some(CardState::Unattached {
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
