#[derive(Debug, Clone)]
pub enum OdinError {
    LexerError(crate::lexer::error::LexerError),
    ParserError(crate::parser::error::ParserError),
}

impl std::fmt::Display for OdinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdinError::LexerError(err) => write!(f, "Lexer error: {err}")?,
            OdinError::ParserError(err) => write!(f, "Parser error: {err}")?,
        }
        Ok(())
    }
}

impl From<crate::lexer::error::LexerError> for OdinError {
    fn from(e: crate::lexer::error::LexerError) -> OdinError {
        OdinError::LexerError(e)
    }
}

impl From<crate::parser::error::ParserError> for OdinError {
    fn from(e: crate::parser::error::ParserError) -> OdinError {
        OdinError::ParserError(e)
    }
}
