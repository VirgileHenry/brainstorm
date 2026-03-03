#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnderControl {
    UnderItsOwnersControl {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UnderYourControl {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl UnderControl {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::UnderItsOwnersControl { span } => *span,
            Self::UnderYourControl { span } => *span,
        }
    }
}

impl UnderControl {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "under its owner's control" => Some(Self::UnderItsOwnersControl {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "under your control" => Some(Self::UnderYourControl {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
