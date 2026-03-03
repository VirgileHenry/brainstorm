#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalZone {
    TheBattlefield {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl GlobalZone {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::TheBattlefield { span } => *span,
        }
    }
}

impl GlobalZone {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "the battlefield" => Some(Self::TheBattlefield {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
