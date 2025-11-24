mod ability_tree_rules;

use crate::parser::node::ParserNode;

pub fn fuse(tokens: &[ParserNode]) -> Option<ParserNode> {
    lazy_static::lazy_static!(
        static ref rule_map: RuleMap = RuleMap::create(&[

        ]);
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
    ( [ $($($elem:tt)+),* ] ) => {
        crate::parser::rules::StateId::new(&[])
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
