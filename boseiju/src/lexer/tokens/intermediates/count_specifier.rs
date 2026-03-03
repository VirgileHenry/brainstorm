#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CountSpecifier {
    All {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AllOthers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Target {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CountSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::All { span } => *span,
            Self::AllOthers { span } => *span,
            Self::Target { span } => *span,
        }
    }
}

impl CountSpecifier {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "all" | "each" => Some(Self::All {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "target" | "targets" => Some(Self::Target {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "all other" | "each other" => Some(Self::AllOthers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
