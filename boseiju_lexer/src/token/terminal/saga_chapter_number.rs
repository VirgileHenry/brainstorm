#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SagaChapterNumber {
    pub chapter: u32,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Default for SagaChapterNumber {
    fn default() -> Self {
        Self {
            chapter: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SagaChapterNumber {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for SagaChapterNumber {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl std::fmt::Display for SagaChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "saga chapter {}", self.chapter)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for SagaChapterNumber {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "i" => Ok(SagaChapterNumber {
                chapter: 1,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ii" => Ok(SagaChapterNumber {
                chapter: 2,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "iii" => Ok(SagaChapterNumber {
                chapter: 3,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "iv" => Ok(SagaChapterNumber {
                chapter: 4,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "v" => Ok(SagaChapterNumber {
                chapter: 5,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vi" => Ok(SagaChapterNumber {
                chapter: 6,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
