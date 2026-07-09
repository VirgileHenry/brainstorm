#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub color: mtg_data::Color,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Color {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::Color::all().map(|color| Self {
            color,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Color {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Color {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;
        Ok(Self {
            color: mtg_data::Color::from_str(&span.text).map_err(|_| ())?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl idris::Idris for Color {
    const COUNT: usize = mtg_data::Color::COUNT;
    fn id(&self) -> usize {
        self.color.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::Color::name_from_id(id)
    }
}
