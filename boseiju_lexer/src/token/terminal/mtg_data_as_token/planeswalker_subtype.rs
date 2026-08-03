/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaneswalkerSubtype {
    pub planeswalker_subtype: mtg_data::PlaneswalkerType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl PlaneswalkerSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::PlaneswalkerType::all().map(|planeswalker_subtype| PlaneswalkerSubtype {
            planeswalker_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for PlaneswalkerSubtype {
    fn default() -> Self {
        Self {
            planeswalker_subtype: mtg_data::PlaneswalkerType::Jace,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlaneswalkerSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for PlaneswalkerSubtype {
    const COUNT: usize = mtg_data::PlaneswalkerType::COUNT;
    fn id(&self) -> usize {
        self.planeswalker_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::PlaneswalkerType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for PlaneswalkerSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            planeswalker_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl std::fmt::Display for PlaneswalkerSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.planeswalker_subtype.fmt(f)
    }
}
