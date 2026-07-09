/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnableZone {
    /// The battlefield is technically not an owned zone.
    ///
    /// However, "the battlefield under some player control" can be interpreted
    /// as "your battlefield ?" soo it makes sense
    Battlefield {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// A Deck is a owned zone that exist before a game starts.
    Deck {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Graveyard {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Hand {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Library {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for OwnableZone {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Battlefield { span } => *span,
            Self::Graveyard { span } => *span,
            Self::Deck { span } => *span,
            Self::Hand { span } => *span,
            Self::Library { span } => *span,
        }
    }
}

impl std::fmt::Display for OwnableZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnableZone::Battlefield { .. } => write!(f, "graveyard"),
            OwnableZone::Deck { .. } => write!(f, "graveyard"),
            OwnableZone::Graveyard { .. } => write!(f, "graveyard"),
            OwnableZone::Hand { .. } => write!(f, "hand"),
            OwnableZone::Library { .. } => write!(f, "library"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for OwnableZone {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "deck" | "decks" | "starting deck" => Ok(OwnableZone::Deck {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "graveyard" | "graveyards" => Ok(OwnableZone::Graveyard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hand" | "hands" => Ok(OwnableZone::Hand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "library" | "libraries" => Ok(OwnableZone::Library {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
