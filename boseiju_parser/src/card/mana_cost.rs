impl crate::card::Parse for boseiju_tree::ability_tree::mana_cost::ManaCost {
    type Source = String;
    type Error = ManaCostParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        let mut symbols = boseiju_tree::HeapArrayVec::new();

        lazy_static::lazy_static!(
            static ref mana_cost_regex: regex::Regex = regex::Regex::new(r"(\{[^{}]+\})")
                .expect("Failed to compile the mana cost iterator regex: {e}");
        );

        for capture in mana_cost_regex.find_iter(&from) {
            let span = boseiju_lexer::LexerSpan {
                start: capture.start(),
                length: capture.len(),
                text: capture.as_str(),
            };
            let mana = boseiju_lexer::terminal::ManaSymbol::try_from(&span)?;
            symbols.push(mana);
        }

        Ok(Self {
            symbols,
            #[cfg(feature = "spanned_tree")]
            span: boseiju_span::Span::from_str(from),
        })
    }
}

#[derive(Debug)]
pub enum ManaCostParseError {
    InvalidManaSymbol(mtg_data::ManaSymbolParseError),
}

impl From<mtg_data::ManaSymbolParseError> for ManaCostParseError {
    fn from(error: mtg_data::ManaSymbolParseError) -> Self {
        Self::InvalidManaSymbol(error)
    }
}

impl std::fmt::Display for ManaCostParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidManaSymbol(error) => write!(f, "Invalid mana symbol: {error}"),
        }
    }
}

impl std::error::Error for ManaCostParseError {}
