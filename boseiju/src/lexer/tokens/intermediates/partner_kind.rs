/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartnerKind {
    CharacterSelect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FatherAndSon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FriendsForever {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Survivors {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for PartnerKind {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CharacterSelect { span } => *span,
            Self::FatherAndSon { span } => *span,
            Self::FriendsForever { span } => *span,
            Self::Survivors { span } => *span,
        }
    }
}

impl PartnerKind {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "character select" => Some(Self::CharacterSelect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "father & son" => Some(Self::FatherAndSon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friends forever" => Some(Self::FriendsForever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "survivor" => Some(Self::Survivors {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
