/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub struct ParserError {
    pub nodes: arrayvec::ArrayVec<super::node::ParserNode, 128>,
    pub best_attempt: arrayvec::ArrayVec<super::node::ParserNode, 128>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

impl std::error::Error for ParserError {}
