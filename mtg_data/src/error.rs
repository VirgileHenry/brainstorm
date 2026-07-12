#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsingError {
    pub item: &'static str,
    pub message: &'static str,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse {}: {}", self.item, self.message)
    }
}

impl std::error::Error for ParsingError {}
