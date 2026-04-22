use crate::lexer::IntoToken;

/// Wrapper around the keyword action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAction {
    pub keyword_action: mtg_data::KeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

#[cfg(feature = "lexer")]
impl IntoToken for KeywordAction {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            keyword_action: crate::utils::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for KeywordAction {
    const COUNT: usize = mtg_data::KeywordAction::COUNT;
    fn id(&self) -> usize {
        self.keyword_action.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::KeywordAction::name_from_id(id)
    }
}
