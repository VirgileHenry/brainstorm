/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbilityKind {
    Ability {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ActivatedAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FinalChapterAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LoyaltyAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TriggeredAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AbilityKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Ability { span } => *span,
            Self::ActivatedAbility { span } => *span,
            Self::FinalChapterAbility { span } => *span,
            Self::LoyaltyAbility { span } => *span,
            Self::TriggeredAbility { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AbilityKind {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "ability" | "abilities" => Ok(Self::Ability {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activated ability" | "activated abilities" => Ok(Self::ActivatedAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "final chapter ability" | "final chapter abilities" => Ok(Self::FinalChapterAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "loyalty ability" => Ok(Self::LoyaltyAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "triggered ability" | "triggered abilities" => Ok(Self::TriggeredAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
