#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotOfAKind {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl NotOfAKind {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "non-" => Some(NotOfAKind {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
