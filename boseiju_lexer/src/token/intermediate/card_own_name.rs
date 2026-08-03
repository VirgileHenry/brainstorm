#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardOwnName {
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardOwnName {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardOwnName {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            /* Fixme: big mistake, he / him / itself can reference some other card */
            "~" | "he" | "she" | "him" | "himself" | "her" | "itself" => Ok(Self {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}

impl idris::Idris for CardOwnName {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "CardOwnName"
    }
}
