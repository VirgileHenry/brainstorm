#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreatureGrouping {
    Outlaw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CreatureGrouping {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Outlaw { span } => *span,
        }
    }
}

impl CreatureGrouping {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "outlaw" | "outlaws" => Some(Self::Outlaw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
