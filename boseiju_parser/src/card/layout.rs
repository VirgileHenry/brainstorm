mod normal;
mod token;

impl crate::card::Parse for boseiju_tree::card::layout::Layout {
    type Source = mtg_cardbase::Card;
    type Error = LayoutParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        match from.layout.as_str() {
            "normal" => Ok(Self::Normal {
                layout: boseiju_tree::card::layout::normal::NormalLayout::parse(from)?,
            }),
            "token" => Ok(Self::Token {
                layout: boseiju_tree::card::layout::token::TokenLayout::parse(from)?,
            }),
            other => Err(LayoutParseError::UnknownLayout {
                layout: other.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum LayoutParseError {
    InvalidColors(crate::card::colors::ColorsParseError),
    InvalidManaCost(crate::card::mana_cost::ManaCostParseError),
    InvalidTypeLine(crate::card::type_line::TypeLineParseError),
    LexerError(boseiju_lexer::LexerError),
    ParserError(crate::ability_tree::ParserError),
    UnknownLayout { layout: String },
}

impl From<crate::card::colors::ColorsParseError> for LayoutParseError {
    fn from(error: crate::card::colors::ColorsParseError) -> Self {
        Self::InvalidColors(error)
    }
}

impl From<crate::card::mana_cost::ManaCostParseError> for LayoutParseError {
    fn from(error: crate::card::mana_cost::ManaCostParseError) -> Self {
        Self::InvalidManaCost(error)
    }
}

impl From<crate::card::type_line::TypeLineParseError> for LayoutParseError {
    fn from(error: crate::card::type_line::TypeLineParseError) -> Self {
        Self::InvalidTypeLine(error)
    }
}

impl From<boseiju_lexer::LexerError> for LayoutParseError {
    fn from(error: boseiju_lexer::LexerError) -> Self {
        Self::LexerError(error)
    }
}

impl From<crate::ability_tree::ParserError> for LayoutParseError {
    fn from(error: crate::ability_tree::ParserError) -> Self {
        Self::ParserError(error)
    }
}

impl std::fmt::Display for LayoutParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidColors(error) => write!(f, "Invalid colors: {error}"),
            Self::InvalidManaCost(error) => write!(f, "Invalid mana_cost: {error}"),
            Self::InvalidTypeLine(error) => write!(f, "Invalid type line: {error}"),
            Self::LexerError(error) => write!(f, "Lexer error: {error}"),
            Self::ParserError(error) => write!(f, "Parser error: {error}"),
            Self::UnknownLayout { layout } => write!(f, "Unknown layout: \"{layout}\""),
        }
    }
}

impl std::error::Error for LayoutParseError {}
