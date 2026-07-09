/// Wrapper around the enchantment subtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstantSorcerySubtype {
    pub instant_sorcery_subtype: mtg_data::SpellType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl InstantSorcerySubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::SpellType::all().map(|spell_subtype| InstantSorcerySubtype {
            instant_sorcery_subtype: spell_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for InstantSorcerySubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for InstantSorcerySubtype {
    const COUNT: usize = mtg_data::SpellType::COUNT;
    fn id(&self) -> usize {
        self.instant_sorcery_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::SpellType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for InstantSorcerySubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            instant_sorcery_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
