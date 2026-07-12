/// Wrapper around the ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbilityWord {
    pub ability_word: mtg_data::AbilityWord,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AbilityWord {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for AbilityWord {
    fn default() -> Self {
        Self {
            ability_word: mtg_data::AbilityWord::Channel,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AbilityWord {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;
        Ok(Self {
            ability_word: mtg_data::AbilityWord::from_str(&span.text).map_err(|_| ())?,
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

impl std::fmt::Display for AbilityWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ability_word.fmt(f)
    }
}
