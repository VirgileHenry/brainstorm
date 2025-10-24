/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub struct ParserError {
    pub nodes: Vec<super::node::ParserNode>,
    pub best_attempt: Vec<super::node::ParserNode>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

impl std::error::Error for ParserError {}
