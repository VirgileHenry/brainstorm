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
                "bands" | "banded" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Banding,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "championed" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Champion,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "crews" | "crewed" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Crew,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "enchanting" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Enchant,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "enlists" | "enlisted" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Enlist,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "evolves" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Evolve,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "foretelling" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Foretell,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "haunts" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Haunt,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "mentors" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Mentor,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "spliced" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Splice,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "stations" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Station,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "suspended" => Some(Self {
                    keyword_ability: mtg_data::KeywordAbility::Suspend,
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
