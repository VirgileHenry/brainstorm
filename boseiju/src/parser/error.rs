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

impl ParserError {
    pub fn keep_best_error(self, other: Option<Self>) -> Self {
        match (self, other) {
            (current, None) => current,
            (current, Some(Self::UnparsableInput { .. })) => current,
            (
                Self::UnexpectedFollowingToken {
                    state_size: self_state_size,
                    current: self_current,
                    next: self_next,
                },
                Some(Self::UnexpectedFollowingToken {
                    state_size: other_state_size,
                    current: other_current,
                    next: other_next,
                }),
            ) => {
                if self_state_size < other_state_size {
                    Self::UnexpectedFollowingToken {
                        state_size: self_state_size,
                        current: self_current,
                        next: self_next,
                    }
                } else {
                    Self::UnexpectedFollowingToken {
                        state_size: other_state_size,
                        current: other_current,
                        next: other_next,
                    }
                }
            }
            (
                Self::UnparsableInput { .. },
                Some(Self::UnexpectedFollowingToken {
                    state_size,
                    current,
                    next,
                }),
            ) => Self::UnexpectedFollowingToken {
                state_size,
                current,
                next,
            },
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

impl std::error::Error for ParserError {}
