mod ability_tree_rules;

use crate::parser::node::ParserNode;

const ALL_RULES: &[&[ParserRule]] = &[ability_tree_rules::ABILITY_TREE_RULES];

pub fn fuse(tokens: &[ParserNode]) -> Option<ParserNode> {
    lazy_static::lazy_static!(
        static ref rule_map: RuleMap = RuleMap::create(ALL_RULES);
    );

    let state_id = StateId::new(tokens);
    let rule = rule_map.rules.get(&state_id)?;
    Some((rule.conversion_func)(tokens))
}

/// The State Id is a unique identifier for a given subslice of ParserNode.
///
/// The idea is that rules can be assigned to State Ids, and they allow fast rule lookup.
/// Furthermore, they can be used to know the allowed tokens before and after other tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateId(usize);

impl StateId {
    const fn new(tokens: &[ParserNode]) -> Self {
        let mut result = 0;
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            result += ParserNode::COUNT.pow(i as u32) * token.id();
            i += 1;
        }
        Self(result)
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
            const TOKENS: &[ParserNode] = &[ $($out)* ];
            crate::parser::rules::StateId::new(TOKENS)
        }
    };
    /* Special case for lexer token, we need to instanciate it */
    ( out = [ $($out:tt)* ], rest = [ ParserNode::LexerToken( $($lt:tt)* ) $(, $($rest:tt)+ )? ] ) => {
        crate::state_id!(out = [ $($out)* ParserNode::LexerToken( $($lt)* ), ], rest = [ $( $($rest)+ )? ])
    };
    /* Otherwise, instanciate the token with uninit */
    ( out = [ $($out:tt)* ], rest = [ ParserNode:: $node:ident ( $($lt:tt)* ) $(, $($rest:tt)+ )? ] ) => {
        crate::state_id!(
            out = [ $($out)* ParserNode:: $node ( unsafe { ::std::mem::MaybeUninit::uninit().assume_init() } ), ],
            rest = [ $( $($rest)+ )? ]
        )
    };
}

/// A single rule of merging nodes together for the parser.
#[derive(Debug, Clone)]
struct ParserRule {
    from_state: StateId,
    conversion_func: fn(&[ParserNode]) -> ParserNode,
}

impl ParserRule {
    pub const fn create(from_state: StateId, conversion_func: fn(&[ParserNode]) -> ParserNode) -> Self {
        Self {
            from_state,
            conversion_func,
        }
    }
}

#[macro_export]
macro_rules! make_parser_rule {
    () => {};
}

/// A given map of rules to merge tokens.
struct RuleMap {
    rules: std::collections::HashMap<StateId, ParserRule>,
}

impl RuleMap {
    fn create(rules: &[&[ParserRule]]) -> Self {
        let mut rule_map = std::collections::HashMap::new();
        for rule in rules.iter().cloned().flatten().cloned() {
            rule_map.insert(rule.from_state, rule);
        }
        Self { rules: rule_map }
    }
}
