/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedTransformation {
    EverflameHeroesLegacy {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fenric {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HumbleMerchant {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LegitimateBuisnessperson {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MilevaTheStalwart {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Moon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    VituGhazi {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for NamedTransformation {
    fn default() -> Self {
        Self::HumbleMerchant {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedTransformation {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::EverflameHeroesLegacy { span } => *span,
            Self::Fenric { span } => *span,
            Self::HumbleMerchant { span } => *span,
            Self::LegitimateBuisnessperson { span } => *span,
            Self::MilevaTheStalwart { span } => *span,
            Self::Moon { span } => *span,
            Self::VituGhazi { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedTransformation::EverflameHeroesLegacy { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::Fenric { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::HumbleMerchant { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::LegitimateBuisnessperson { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::MilevaTheStalwart { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::Moon { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::VituGhazi { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedTransformation {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "everflame, heroes' legacy" => Ok(NamedTransformation::EverflameHeroesLegacy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fenric" => Ok(NamedTransformation::Fenric {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "humble merchant" => Ok(NamedTransformation::HumbleMerchant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legitimate businessperson" => Ok(NamedTransformation::LegitimateBuisnessperson {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mileva, the stalwart" => Ok(NamedTransformation::MilevaTheStalwart {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "moon" => Ok(NamedTransformation::Moon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vitu-ghazi" => Ok(NamedTransformation::VituGhazi {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
