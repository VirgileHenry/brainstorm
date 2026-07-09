/// Wrapper around the keyword ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAbility {
    pub keyword_ability: mtg_data::KeywordAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for KeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for KeywordAbility {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;
        if let Ok(keyword_ability) = mtg_data::KeywordAbility::from_str(&span.text) {
            Ok(Self {
                keyword_ability,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            match span.text {
                "bands" | "banded" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Banding,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "championed" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Champion,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "crews" | "crewed" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Crew,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "embalmed" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Embalm,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "enchanting" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Enchant,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "enlists" | "enlisted" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Enlist,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "evolves" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Evolve,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "foretelling" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Foretell,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "haunts" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Haunt,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "kicks" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Kicker,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "mentors" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Mentor,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "spliced" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Splice,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "stations" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Station,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "suspended" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Suspend,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "trains" => Ok(Self {
                    keyword_ability: mtg_data::KeywordAbility::Training,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => Err(()),
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
