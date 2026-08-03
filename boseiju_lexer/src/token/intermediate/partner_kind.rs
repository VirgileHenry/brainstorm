/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartnerKind {
    CharacterSelect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FatherAndSon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FriendsForever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Survivors {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PartnerKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CharacterSelect { span } => *span,
            Self::FatherAndSon { span } => *span,
            Self::FriendsForever { span } => *span,
            Self::Survivors { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for PartnerKind {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "character select" => Ok(Self::CharacterSelect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "father & son" => Ok(Self::FatherAndSon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friends forever" => Ok(Self::FriendsForever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "survivor" => Ok(Self::Survivors {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
