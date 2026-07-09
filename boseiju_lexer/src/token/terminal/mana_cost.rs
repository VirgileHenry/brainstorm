pub const MAX_MANA_SYMBOL_PER_MANA_COST: usize = 10;

/// A cost requiring mana.
///
/// See also <https://mtg.fandom.com/wiki/Mana_cost>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    pub cost: arrayvec::ArrayVec<crate::token::terminal::ManaSymbol, MAX_MANA_SYMBOL_PER_MANA_COST>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl ManaCost {
    pub fn mana_value(&self) -> usize {
        self.cost.iter().map(|mana| mana.mana_value()).sum()
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaCost {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ManaCost {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        let mut cost = arrayvec::ArrayVec::new();

        /* Yeah, yeah, it's not that hard and may not need a regex. Whatever for now. */
        lazy_static::lazy_static!(
            static ref mana_cost_regex: regex::Regex = regex::Regex::new(r"(\{[^{}]+\})")
                .expect("Failed to compile the mana cost iterator regex: {e}");
        );

        for capture in mana_cost_regex.captures_iter(&span.text) {
            let span = crate::LexerSpan {
                start: span.start + capture.get_match().start(),
                length: capture.get_match().len(),
                text: capture.get_match().as_str(),
            };
            let mana = crate::token::terminal::ManaSymbol::try_from(&span)?;
            cost.push(mana);
        }

        Ok(ManaCost {
            cost,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [crate::token::terminal::ManaSymbol];
    fn deref(&self) -> &Self::Target {
        self.cost.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cost.as_mut_slice()
    }
}
