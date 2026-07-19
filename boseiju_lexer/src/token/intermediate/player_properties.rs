#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerProperties {
    Devotion {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HandSize {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LifeTotal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MaximumHandSize {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MinimumDeckSize {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Speed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingDeck {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingHandSize {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingLifeTotal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OpeningHand {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerProperties {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Devotion { span } => *span,
            Self::HandSize { span } => *span,
            Self::LifeTotal { span } => *span,
            Self::MaximumHandSize { span } => *span,
            Self::MinimumDeckSize { span } => *span,
            Self::Speed { span } => *span,
            Self::StartingDeck { span } => *span,
            Self::StartingHandSize { span } => *span,
            Self::StartingLifeTotal { span } => *span,
            Self::OpeningHand { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for PlayerProperties {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "devotion" => Ok(Self::Devotion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hand size" => Ok(Self::HandSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life total" => Ok(Self::LifeTotal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "maximum hand size" => Ok(Self::MaximumHandSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "minimum deck size" => Ok(Self::MinimumDeckSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "speed" => Ok(Self::Speed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting deck" => Ok(Self::StartingDeck {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting hand size" => Ok(Self::StartingHandSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting life total" => Ok(Self::StartingLifeTotal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "opening hand" => Ok(Self::OpeningHand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
