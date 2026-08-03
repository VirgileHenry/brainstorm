#[derive(Debug)]
pub enum CardParseError {
    InvalidJson(CardJsonError),
    InvalidLayout(crate::card::LayoutParseError),
}

impl From<CardJsonError> for CardParseError {
    fn from(error: CardJsonError) -> Self {
        Self::InvalidJson(error)
    }
}

impl From<crate::card::LayoutParseError> for CardParseError {
    fn from(error: crate::card::LayoutParseError) -> Self {
        Self::InvalidLayout(error)
    }
}

impl std::fmt::Display for CardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(error) => write!(f, "Invalid card JSON: {error}"),
            Self::InvalidLayout(error) => write!(f, "Invalid card layout: {error}"),
        }
    }
}

impl std::error::Error for CardParseError {}

#[derive(Debug)]
pub enum CardJsonError {
    InvalidColors(crate::card::colors::ColorsParseError),
    InvalidLegality(crate::card::legalities::LegalitiesParseError),
    InvalidUuid(uuid::Error),
}

impl std::fmt::Display for CardJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidColors(error) => write!(f, "Invalid colors: {error}"),
            Self::InvalidLegality(error) => write!(f, "Invalid legality: {error}"),
            Self::InvalidUuid(error) => write!(f, "Invalid UUID: {error}"),
        }
    }
}

impl std::error::Error for CardJsonError {}
