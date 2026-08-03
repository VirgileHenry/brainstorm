/// Wrapper around the mana symbol type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManaSymbol {
    pub mana_symbol: mtg_data::ManaSymbol,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl ManaSymbol {
    pub fn mana_value(&self) -> usize {
        self.mana_symbol.mana_value()
    }
}

impl Default for ManaSymbol {
    fn default() -> Self {
        Self {
            mana_symbol: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaSymbol {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ManaSymbol {
    type Error = mtg_data::ManaSymbolParseError;
    fn try_from(span: &crate::LexerSpan) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        Ok(Self {
            mana_symbol: mtg_data::ManaSymbol::from_str(span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for ManaSymbol {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "ManaSymbol"
    }
}

impl std::fmt::Display for ManaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.mana_symbol.fmt(f)
    }
}
