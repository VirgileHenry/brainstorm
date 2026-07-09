/// Wrapper around the mana symbol type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManaSymbol {
    mana: mtg_data::ManaSymbol,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl ManaSymbol {
    pub fn mana_value(&self) -> usize {
        self.mana.mana_value()
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaSymbol {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ManaSymbol {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;
        Ok(Self {
            mana: mtg_data::ManaSymbol::from_str(span.text).map_err(|_| ())?,
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
