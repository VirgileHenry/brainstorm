/// Wrapper around the battle subtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BattleSubtype {
    pub battle_subtype: mtg_data::BattleType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl BattleSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::BattleType::all().map(|battle_subtype| BattleSubtype {
            battle_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for BattleSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for BattleSubtype {
    const COUNT: usize = mtg_data::BattleType::COUNT;
    fn id(&self) -> usize {
        self.battle_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::BattleType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for BattleSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            battle_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
