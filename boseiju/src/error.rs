#[derive(Debug, Clone)]
pub enum BoseijuError {
    LexerError(crate::lexer::LexerError),
    ParserError(crate::parser::ParserError),
}

impl std::fmt::Display for BoseijuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoseijuError::LexerError(err) => write!(f, "Lexer error: {err}")?,
            BoseijuError::ParserError(err) => write!(f, "Parser error: {err}")?,
        }
        Ok(())
    }
}

impl From<crate::lexer::LexerError> for BoseijuError {
    fn from(e: crate::lexer::LexerError) -> BoseijuError {
        BoseijuError::LexerError(e)
    }
}

impl From<crate::parser::ParserError> for BoseijuError {
    fn from(e: crate::parser::ParserError) -> BoseijuError {
        BoseijuError::ParserError(e)
    }
}
