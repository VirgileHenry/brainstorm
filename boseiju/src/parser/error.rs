/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedFollowingToken {
        state_size: usize,
        current: super::node::ParserNode,
        next: super::node::ParserNode,
    },
    UnparsableInput {
        nodes: Vec<crate::lexer::tokens::TokenKind>,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

impl std::error::Error for ParserError {}
