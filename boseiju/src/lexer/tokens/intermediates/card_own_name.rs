#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardOwnName {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl CardOwnName {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            /* Fixme: big mistake, he / him / itself can reference some other card */
            "~" | "he" | "she" | "him" | "himself" | "her" | "itself" => Some(Self {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl idris::Idris for CardOwnName {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "CardOwnName"
    }
}
