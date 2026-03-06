use crate::lexer::IntoToken;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SagaChapterNumber {
    pub chapter: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl SagaChapterNumber {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for SagaChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "saga chapter {}", self.chapter)
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for SagaChapterNumber {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "i" => Some(SagaChapterNumber {
                chapter: 1,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ii" => Some(SagaChapterNumber {
                chapter: 2,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "iii" => Some(SagaChapterNumber {
                chapter: 3,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "iv" => Some(SagaChapterNumber {
                chapter: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "v" => Some(SagaChapterNumber {
                chapter: 5,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vi" => Some(SagaChapterNumber {
                chapter: 6,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
