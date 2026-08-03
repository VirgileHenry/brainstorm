/// Wrapper around the ability word.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FlavorWord {
    pub flavor_word: mtg_data::FlavorWord,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl FlavorWord {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::FlavorWord::all().map(|ability_word| FlavorWord {
            flavor_word: ability_word,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for FlavorWord {
    fn default() -> Self {
        Self {
            flavor_word: mtg_data::FlavorWord::Fight,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for FlavorWord {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for FlavorWord {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;
        if let Ok(flavor_word) = mtg_data::FlavorWord::from_str(&span.text) {
            Ok(Self {
                flavor_word,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            match span.text {
                "~'s kiss" => Ok(Self {
                    flavor_word: mtg_data::FlavorWord::GenestealersKiss,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "~ entity" => Ok(Self {
                    flavor_word: mtg_data::FlavorWord::MidnightEntity,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "~ my love" => Ok(Self {
                    flavor_word: mtg_data::FlavorWord::EdEMyLove,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => Err(()),
            }
        }
    }
}

impl idris::Idris for FlavorWord {
    const COUNT: usize = mtg_data::AbilityWord::COUNT;
    fn id(&self) -> usize {
        self.flavor_word.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::AbilityWord::name_from_id(id)
    }
}

impl std::fmt::Display for FlavorWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.flavor_word.fmt(f)
    }
}
