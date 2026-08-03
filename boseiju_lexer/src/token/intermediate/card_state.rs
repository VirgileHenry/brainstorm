#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardState {
    Able {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Amassed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Attached {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Bargained {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Blocking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Blocked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Convoked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Countered {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DeclaredAs {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Devoured {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Enchanted {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Equipped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Escaped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    EvidenceWasCollected {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exiled {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exploited {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Foretold {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Goaded {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Kicked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Locked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LookedAt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Modified {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Monstrous {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MostRecentlyCast {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mutated {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Paired {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PhasedOut {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Plotted {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Prepared {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Remains {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Renowned {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Revealed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RingBearer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Saddled {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Suspected {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Targeted {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGiftWasPromised {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGiftWasntPromised {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Transformed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unattached {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unblocked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unlocked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unprepared {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Untapped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Warped {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardState {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Able { span } => *span,
            Self::Amassed { span } => *span,
            Self::Attached { span } => *span,
            Self::Attacking { span } => *span,
            Self::Bargained { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span } => *span,
            Self::Convoked { span } => *span,
            Self::Countered { span } => *span,
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
            Self::Kicked { span } => *span,
            Self::Locked { span } => *span,
            Self::LookedAt { span } => *span,
            Self::Modified { span } => *span,
            Self::Monstrous { span } => *span,
            Self::MostRecentlyCast { span } => *span,
            Self::Mutated { span } => *span,
            Self::Paired { span } => *span,
            Self::PhasedOut { span } => *span,
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
            Self::Unlocked { span } => *span,
            Self::Unprepared { span } => *span,
            Self::Untapped { span } => *span,
            Self::Warped { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardState {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "able" => Ok(CardState::Able {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amassed" => Ok(CardState::Amassed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attached" => Ok(CardState::Attached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacking" | "attacker" => Ok(CardState::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bargained" => Ok(CardState::Bargained {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocking" => Ok(CardState::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocked" => Ok(CardState::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "declared as" => Ok(CardState::DeclaredAs {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "convoked" => Ok(CardState::Convoked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "countered" => Ok(CardState::Countered {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "devoured" => Ok(CardState::Devoured {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted" => Ok(CardState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "equipped" => Ok(CardState::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "escaped" => Ok(CardState::Escaped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evidence was collected" => Ok(CardState::EvidenceWasCollected {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exiled" => Ok(CardState::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exploited" => Ok(CardState::Exploited {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "foretold" => Ok(CardState::Foretold {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "goaded" => Ok(CardState::Goaded {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kicked" => Ok(CardState::Kicked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "locked" => Ok(CardState::Locked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "looked at" => Ok(CardState::LookedAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "modified" => Ok(CardState::Modified {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "monstrous" => Ok(CardState::Monstrous {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "most recently cast" => Ok(CardState::MostRecentlyCast {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mutated" => Ok(CardState::Mutated {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "paired" => Ok(CardState::Paired {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phased-out" => Ok(CardState::PhasedOut {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plotted" => Ok(CardState::Plotted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prepared" => Ok(CardState::Prepared {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "remain" | "remains" => Ok(CardState::Remains {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "renowned" => Ok(CardState::Renowned {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ring-bearer" | "ring-bearers" => Ok(CardState::RingBearer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "revealed" => Ok(CardState::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "saddled" | "saddles" => Ok(CardState::Saddled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "suspected" => Ok(CardState::Suspected {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Ok(CardState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "targeted" => Ok(CardState::Targeted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the gift was promised" => Ok(CardState::TheGiftWasPromised {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the gift wasn't promised" => Ok(CardState::TheGiftWasntPromised {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "transformed" => Ok(CardState::Transformed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unattached" => Ok(CardState::Unattached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unblocked" => Ok(CardState::Unblocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unlocked" => Ok(Self::Unlocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unprepared" => Ok(CardState::Unprepared {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "untapped" => Ok(CardState::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "warped" => Ok(CardState::Warped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
