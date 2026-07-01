use crate::lexer::IntoToken;

/// Wrapper around the keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAbility {
    pub keyword_ability: mtg_data::KeywordAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

#[cfg(feature = "lexer")]
impl IntoToken for KeywordAbility {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;
        if let Ok(keyword_ability) = mtg_data::KeywordAbility::from_str(&span.text) {
            Some(Self {
                keyword_ability,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            match span.text {
                "crews" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Crew,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => None,
            }
        }
    }
}

impl idris::Idris for KeywordAbility {
    const COUNT: usize = mtg_data::KeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_ability.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::KeywordAbility::name_from_id(id)
    }
}
