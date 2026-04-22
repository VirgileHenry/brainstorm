use crate::lexer::IntoToken;

/// Wrapper around the ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbilityWord {
    pub ability_word: mtg_data::AbilityWord,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

#[cfg(feature = "lexer")]
impl IntoToken for AbilityWord {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        Some(Self {
            ability_word: crate::utils::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for AbilityWord {
    const COUNT: usize = mtg_data::AbilityWord::COUNT;
    fn id(&self) -> usize {
        self.ability_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::AbilityWord::name_from_id(id)
    }
}
