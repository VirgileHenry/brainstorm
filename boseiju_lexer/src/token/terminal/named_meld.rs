/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedMeld {
    HanweirTheWrithingTownship {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MishraLostToPhyrexia {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TitaniaGaeaIncarnate {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RagnarokDivineDeliverance {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChitteringHost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UrzaPlaneswalker {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BriselaVoiceOfNightmares {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedMeld {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::HanweirTheWrithingTownship { span } => *span,
            Self::MishraLostToPhyrexia { span } => *span,
            Self::TitaniaGaeaIncarnate { span } => *span,
            Self::RagnarokDivineDeliverance { span } => *span,
            Self::ChitteringHost { span } => *span,
            Self::UrzaPlaneswalker { span } => *span,
            Self::BriselaVoiceOfNightmares { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedMeld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedMeld::HanweirTheWrithingTownship { .. } => write!(f, "hanweir, the writhing township"),
            NamedMeld::MishraLostToPhyrexia { .. } => write!(f, "mishra, lost to phyrexia"),
            NamedMeld::TitaniaGaeaIncarnate { .. } => write!(f, "titania, gaea incarnate"),
            NamedMeld::RagnarokDivineDeliverance { .. } => write!(f, "ragnarok, divine deliverance"),
            NamedMeld::ChitteringHost { .. } => write!(f, "chittering host"),
            NamedMeld::UrzaPlaneswalker { .. } => write!(f, "urza, planeswalker"),
            NamedMeld::BriselaVoiceOfNightmares { .. } => write!(f, "brisela, voice of nightmares"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedMeld {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "hanweir, the writhing township" => Ok(NamedMeld::HanweirTheWrithingTownship {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mishra, lost to phyrexia" => Ok(NamedMeld::MishraLostToPhyrexia {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "titania, gaea incarnate" => Ok(NamedMeld::TitaniaGaeaIncarnate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ragnarok, divine deliverance" => Ok(NamedMeld::RagnarokDivineDeliverance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chittering host" => Ok(NamedMeld::ChitteringHost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "urza, planeswalker" => Ok(NamedMeld::UrzaPlaneswalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brisela, voice of nightmares" => Ok(NamedMeld::BriselaVoiceOfNightmares {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
