#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonKind {
    Non {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonArtifact {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonAttacking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonBasic {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonBlack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonBlocking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonBlue {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonCommander {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonCreature {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonEnchantment {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonGreen {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonLand {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonLegendary {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonRed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonSnow {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonToken {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NonWhite {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NonKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Non { span } => *span,
            Self::NonArtifact { span } => *span,
            Self::NonAttacking { span } => *span,
            Self::NonBasic { span } => *span,
            Self::NonBlack { span } => *span,
            Self::NonBlocking { span } => *span,
            Self::NonBlue { span } => *span,
            Self::NonCommander { span } => *span,
            Self::NonCreature { span } => *span,
            Self::NonEnchantment { span } => *span,
            Self::NonGreen { span } => *span,
            Self::NonLand { span } => *span,
            Self::NonLegendary { span } => *span,
            Self::NonRed { span } => *span,
            Self::NonSnow { span } => *span,
            Self::NonToken { span } => *span,
            Self::NonWhite { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NonKind {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "non-" => Ok(Self::Non {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonartifact" => Ok(Self::NonArtifact {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonattacking" => Ok(Self::NonAttacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonbasic" => Ok(Self::NonBasic {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonblack" => Ok(Self::NonBlack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonblocking" => Ok(Self::NonBlocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonblue" => Ok(Self::NonBlue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncommander" => Ok(Self::NonCommander {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncreature" => Ok(Self::NonCreature {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonenchantment" => Ok(Self::NonEnchantment {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nongreen" => Ok(Self::NonGreen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonland" => Ok(Self::NonLand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonlegendary" => Ok(Self::NonLegendary {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonred" => Ok(Self::NonRed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonsnow" => Ok(Self::NonSnow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nontoken" => Ok(Self::NonToken {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonwhite" => Ok(Self::NonWhite {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
