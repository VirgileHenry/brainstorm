use crate::parser::node::ParserNode;
use crate::parser::node::ParserNodeKind;

/// The State Id is a unique identifier for a given subslice of ParserNode.
///
/// The idea is that rules can be assigned to State Ids, and they allow fast rule lookup.
/// Furthermore, they can be used to know the allowed tokens before and after other tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId {
    pub size: usize,
    pub ids: [u16; Self::MAX_TOKENS],
}

impl StateId {
    /// Maximum amount of tokens allowed in a single state for a rule.
    /// It can be incremented if required, but the smaller, the faster.
    const MAX_TOKENS: usize = 8;

    pub fn new(tokens: &[ParserNode]) -> Self {
        if tokens.len() > Self::MAX_TOKENS {
            panic!(
                "StateId has a hard limit of {} tokens, found {}",
                Self::MAX_TOKENS,
                tokens.len()
            );
        }
        let mut ids = [u16::MAX; Self::MAX_TOKENS];
        for (i, token) in tokens.iter().enumerate() {
            ids[i] = token.id() as u16;
        }
        Self { size: tokens.len(), ids }
    }

    pub const fn from_kinds(tokens: &[ParserNodeKind]) -> Self {
        if tokens.len() > Self::MAX_TOKENS {
            panic!("StateId has a hard limit of 8 tokens");
        }
        let mut ids = [u16::MAX; Self::MAX_TOKENS];
        let mut i = 0;
        while i < tokens.len() {
            ids[i] = tokens[i].id() as u16;
            i += 1;
        }
        Self { size: tokens.len(), ids }
    }

    pub fn first(&self) -> u16 {
        self.ids[0]
    }

    pub fn last(&self) -> u16 {
        self.ids[self.size - 1]
    }
}

#[macro_export]
macro_rules! state_id {
    /* Entry point, init the recursive call */
    ( [ $($tokens:tt)+ ] ) => {
        crate::state_id!(out = [], rest = [ $($tokens)+ ])
    };
    /* Terminating condition, the entire input is parsed */
    ( out = [ $($out:tt)* ], rest = [ ] ) => {
        {
            const TOKENS: &[crate::parser::node::ParserNodeKind] = &[ $($out)* ];
            crate::parser::rules::StateId::from_kinds(TOKENS)
        }
    };
    /* Special case for lexer token, we need to instanciate it */
    ( out = [ $($out:tt)* ], rest = [ ParserNode::LexerToken( $lt:expr ) $(, $($rest:tt)+ )? ] ) => {
        crate::state_id!(out = [ $($out)* crate::parser::node::ParserNodeKind::LexerToken( $lt ), ], rest = [ $( $($rest)+ )? ])
    };
    /* Otherwise, instanciate the token with uninit */
    ( out = [ $($out:tt)* ], rest = [ ParserNode:: $node:ident ( $($lt:tt)* ) $(, $($rest:tt)+ )? ] ) => {
        crate::state_id!(
            out = [ $($out)* crate::parser::node::ParserNodeKind:: $node , ],
            rest = [ $( $($rest)+ )? ]
        )
    };
}
