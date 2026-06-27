#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonKind {
    NonBlack {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonBlue {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonGreen {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonRed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonWhite {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonBasic {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonCreature {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonLand {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NonToken {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl NonKind {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::NonBlack { span } => *span,
            Self::NonBlue { span } => *span,
            Self::NonGreen { span } => *span,
            Self::NonRed { span } => *span,
            Self::NonWhite { span } => *span,
            Self::NonBasic { span } => *span,
            Self::NonCreature { span } => *span,
            Self::NonLand { span } => *span,
            Self::NonToken { span } => *span,
        }
    }
}

impl NonKind {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "nonblack" => Some(Self::NonBlack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonblue" => Some(Self::NonBlue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nongreen" => Some(Self::NonGreen {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonred" => Some(Self::NonRed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonwhite" => Some(Self::NonWhite {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonbasic" => Some(Self::NonBasic {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncreature" => Some(Self::NonCreature {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nonland" => Some(Self::NonLand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nontoken" => Some(Self::NonToken {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
