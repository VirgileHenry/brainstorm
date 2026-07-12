/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LandSubtype {
    pub land_subtype: mtg_data::LandType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl LandSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::LandType::all().map(|land_subtype| LandSubtype {
            land_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for LandSubtype {
    fn default() -> Self {
        Self {
            land_subtype: mtg_data::LandType::Mountain,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LandSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for LandSubtype {
    const COUNT: usize = mtg_data::LandType::COUNT;
    fn id(&self) -> usize {
        self.land_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::LandType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for LandSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            land_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl std::fmt::Display for LandSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.land_subtype.fmt(f)
    }
}
