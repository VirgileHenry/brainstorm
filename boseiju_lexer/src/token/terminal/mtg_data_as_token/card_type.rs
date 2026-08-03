/// Wrapper around the card type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardType {
    pub card_type: mtg_data::CardType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl CardType {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::CardType::all().map(|card_type| CardType {
            card_type,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for CardType {
    fn default() -> Self {
        Self {
            card_type: mtg_data::CardType::Creature,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardType {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for CardType {
    const COUNT: usize = mtg_data::CardType::COUNT;
    fn id(&self) -> usize {
        self.card_type.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::CardType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardType {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        if let Ok(card_type) = crate::parsing::from_str_singular_or_plural(&span.text) {
            Ok(Self {
                card_type,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            match span.text {
                "bosses" => Ok(Self {
                    card_type: mtg_data::CardType::Boss,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "sorceries" => Ok(Self {
                    card_type: mtg_data::CardType::Sorcery,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => Err(()),
            }
        }
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.card_type.fmt(f)
    }
}
