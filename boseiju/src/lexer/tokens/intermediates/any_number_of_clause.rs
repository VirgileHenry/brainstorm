#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyNumberOfClause {
    #[cfg(feature = "spanned_tree")]
    span: crate::ability_tree::span::TreeSpan,
}

impl AnyNumberOfClause {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "a deck can have any number of cards named ~." => Some(Self {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
