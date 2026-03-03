#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerProperties {
    HandSize {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LifeTotal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MaximumHandSize {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartingLifeTotal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OpeningHand {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl PlayerProperties {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "hand size" => Some(Self::HandSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life total" => Some(Self::LifeTotal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "maxmimum hand size" => Some(Self::MaximumHandSize {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting life total" => Some(Self::StartingLifeTotal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "opening hand" => Some(Self::OpeningHand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
