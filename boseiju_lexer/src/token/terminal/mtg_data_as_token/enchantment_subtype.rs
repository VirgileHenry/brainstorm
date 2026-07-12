/// Wrapper around the enchantment subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EnchantmentSubtype {
    pub enchantment_subtype: mtg_data::EnchantmentType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl EnchantmentSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::EnchantmentType::all().map(|enchantment_subtype| EnchantmentSubtype {
            enchantment_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for EnchantmentSubtype {
    fn default() -> Self {
        Self {
            enchantment_subtype: mtg_data::EnchantmentType::Shard,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnchantmentSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for EnchantmentSubtype {
    const COUNT: usize = mtg_data::EnchantmentType::COUNT;
    fn id(&self) -> usize {
        self.enchantment_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::EnchantmentType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for EnchantmentSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            enchantment_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
