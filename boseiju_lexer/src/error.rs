/// Errors that can be thrown by the lexer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexerError {
    NoTokenMatch { start: usize, end: usize, tokens: String },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::NoTokenMatch {
                start, tokens: message, ..
            } => {
                write!(f, "At character {start}, no tokens matched for: \"{message}\"")
            }
        }
    }
}

impl std::error::Error for LexerError {}
