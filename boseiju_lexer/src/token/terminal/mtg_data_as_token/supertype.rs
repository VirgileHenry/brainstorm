/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Supertype {
    pub supertype: mtg_data::Supertype,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Supertype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::Supertype::all().map(|supertype| Supertype {
            supertype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for Supertype {
    fn default() -> Self {
        Self {
            supertype: mtg_data::Supertype::Legendary,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Supertype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for Supertype {
    const COUNT: usize = mtg_data::Supertype::COUNT;
    fn id(&self) -> usize {
        self.supertype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::Supertype::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Supertype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            supertype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
