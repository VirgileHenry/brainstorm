#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormatSpecific {
    Ante {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChaosEnsue {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DraftRound {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlanarDeck {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Team {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Teammate {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UnopenedBoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for FormatSpecific {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Ante { span } => *span,
            Self::BoosterPack { span } => *span,
            Self::ChaosEnsue { span } => *span,
            Self::DraftRound { span } => *span,
            Self::PlanarDeck { span } => *span,
            Self::Team { span } => *span,
            Self::Teammate { span } => *span,
            Self::UnopenedBoosterPack { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for FormatSpecific {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "ante" => Ok(Self::Ante {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "booster pack" | "booster packs" => Ok(Self::BoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chaos ensue" | "chaos ensues" => Ok(Self::ChaosEnsue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draft round" => Ok(Self::DraftRound {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planar deck" => Ok(Self::PlanarDeck {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "team" => Ok(Self::Team {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "teammate" | "teammates" => Ok(Self::Teammate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unopened booster pack" => Ok(Self::UnopenedBoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
