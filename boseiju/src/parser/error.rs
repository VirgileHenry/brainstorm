/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedFollowingToken {
        current: super::node::ParserNode,
        next: super::node::ParserNode,
        current_best: Vec<super::node::ParserNode>,
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
                    current: self_current,
                    next: self_next,
                    current_best: self_current_best,
                },
                Some(Self::UnexpectedFollowingToken {
                    current: other_current,
                    next: other_next,
                    current_best: other_current_best,
                }),
            ) => {
                if self_current_best.len() < other_current_best.len() {
                    Self::UnexpectedFollowingToken {
                        current: self_current,
                        next: self_next,
                        current_best: self_current_best,
                    }
                } else {
                    Self::UnexpectedFollowingToken {
                        current: other_current,
                        next: other_next,
                        current_best: other_current_best,
                    }
                }
            }
            (
                Self::UnparsableInput { .. },
                Some(Self::UnexpectedFollowingToken {
                    current,
                    next,
                    current_best,
                }),
            ) => Self::UnexpectedFollowingToken {
                current,
                next,
                current_best,
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
