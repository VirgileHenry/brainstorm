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
        match self {
            ParserError::UnexpectedFollowingToken { current, next, .. } => {
                write!(f, "Node {current:?} can't be followed by {next:?}")?;
            }
            ParserError::UnparsableInput { nodes } => {
                write!(f, "Unparsable nodes: {nodes:?}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for ParserError {}
