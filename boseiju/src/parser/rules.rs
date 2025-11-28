mod ability_tree;
mod continuous_effects;
mod imperative;
mod keyword_to_ability;
mod object_references;
mod object_specifiers;
mod statement;
mod trigger_condition;

use crate::parser::node::ParserNode;
use crate::parser::node::ParserNodeKind;

const ALL_RULES: &[&[ParserRule]] = &[
    ability_tree::ABILITY_TREE_RULES,
    continuous_effects::CONTINUOUS_EFFECTS_RULES,
    imperative::IMPERATIVE_RULES,
    keyword_to_ability::KEYWORD_TO_ABILITY_RULES,
    object_references::OBJECT_REFERENCES_RULES,
    object_specifiers::OBJECT_SPECIFIER_RULES,
    statement::STATEMENT_RULES,
    trigger_condition::TRIGGER_CONDITION_RULES,
];

/// The State Id is a unique identifier for a given subslice of ParserNode.
///
/// The idea is that rules can be assigned to State Ids, and they allow fast rule lookup.
/// Furthermore, they can be used to know the allowed tokens before and after other tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateId {
    size: usize,
    ids: [u16; Self::MAX_TOKENS],
}

impl StateId {
    /// Maximum amount of tokens allowed in a single state for a rule.
    /// It can be incremented if required, but the smaller, the faster.
    const MAX_TOKENS: usize = 8;

    fn new(tokens: &[ParserNode]) -> Self {
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

    const fn from_kinds(tokens: &[ParserNodeKind]) -> Self {
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

#[derive(Debug, Clone)]
pub struct ParserRuleDeclarationLocation {
    file: &'static str,
    line: u32,
}

impl std::fmt::Display for ParserRuleDeclarationLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

/// A single rule of merging nodes together for the parser.
#[derive(Debug, Clone)]
struct ParserRule {
    state: StateId,
    result: usize,
    conversion_func: fn(&[ParserNode]) -> Option<ParserNode>,
    creation_loc: ParserRuleDeclarationLocation,
}

impl ParserRule {
    pub const fn create(
        from_state: StateId,
        to_state: usize,
        conversion_func: fn(&[ParserNode]) -> Option<ParserNode>,
        creation_loc: ParserRuleDeclarationLocation,
    ) -> Self {
        Self {
            state: from_state,
            result: to_state,
            conversion_func,
            creation_loc,
        }
    }
}

#[macro_export]
macro_rules! make_parser_rule {
    ( [ $($tokens:tt)+ ] => Some( ParserNode:: $node:ident ( { $($gen:tt)+ } ) ) ) => {
        crate::parser::rules::ParserRule::create(
            crate::state_id!( [ $($tokens)+ ] ),
            crate::parser::node::ParserNodeKind::$node.id(),
            |tokens| match tokens {
                [ $($tokens)+ ] => Some( ParserNode:: $node ( { $($gen)+ } )),
                _ => None,
            },
            crate::parser::rules::ParserRuleDeclarationLocation {
                file: std::file!(),
                line: std::line!(),
            },
        )
    };
}

/// A given map of rules to merge tokens.
pub struct RuleMap {
    /// Number of tokens in the rule wth the biggest number of tokens.
    /// This allow to avoid calling fuse unnecessarily on windows that are too large.
    pub max_rule_size: usize,
    /// List of the rules we have for merging.
    /// For each state id (list of tokens), there is a rule that merge these tokens.
    /// Note that the rule may fail to merge the tokens,
    ///
    /// and the presence of a rule in the HashMap for a given state id does not guarantee
    /// that the tokens will be fused.
    fuse_rules: std::collections::HashMap<StateId, ParserRule>,
    /// For each token id, we store the list of tokens that can be found after it,
    /// following our rules.
    /// This allows to quickly find that our state will fail,
    /// without having to explore it entirely.
    allowed_succeeding: std::collections::HashMap<usize, std::collections::HashSet<usize>>,
}

impl RuleMap {
    pub fn default() -> Result<Self, RuleMapCreationError> {
        Self::create(ALL_RULES)
    }

    fn create(rules: &[&[ParserRule]]) -> Result<Self, RuleMapCreationError> {
        use std::collections::HashMap;
        use std::collections::HashSet;

        /*
            Create a map for fuse rules.
            This map will give, for any state represented as ids of tokens, a rule to fuse those tokens.
        */
        let mut fuse_rules: HashMap<StateId, ParserRule> = HashMap::new();
        for rule in rules.iter().cloned().flatten().cloned() {
            match fuse_rules.get(&rule.state) {
                Some(prev) => {
                    return Err(RuleMapCreationError::DuplicateRule {
                        rule1_loc: prev.creation_loc.clone(),
                        rule2_loc: rule.creation_loc,
                    });
                }
                None => {
                    fuse_rules.insert(rule.state, rule);
                }
            }
        }

        /*
            The max rule size allows the user of the rule map to avoid trying to fuse too much tokens.
            Any attempt to fuse a token count above our max will fail anyway, so we can avoid them.
        */
        let max_rule_size = fuse_rules.keys().map(|k| k.size).max().unwrap_or(0);

        /*
            We then create a map that, for each given token with id A, store the set of token ids that are "allowed" after it.
            A token being "allowed" after another means that, at some point, a rule will be able to fuse the tokens.
        */
        let mut allowed_succeeding: HashMap<usize, HashSet<usize>> = HashMap::new();

        /*
            The first step to build such a map is to:
            1)  Iterate through each rules, and for each tokens A, B succeeding each other in a rule,
                it means that B is allowed to succeed A, since there will be a rule to merge them.
        */
        for state in fuse_rules.keys() {
            /* For each rule, iterate through succeeding tokens, and set the follower to be allowed to follow the followee. */
            for window in state.ids.windows(2).take(state.size.saturating_sub(1)) {
                let (current, next) = match window {
                    [c, n] => (usize::from(*c), usize::from(*n)),
                    _ => unreachable!(),
                };
                match allowed_succeeding.get_mut(&current) {
                    Some(set) => {
                        set.insert(next);
                    }
                    None => {
                        let mut set = HashSet::new();
                        set.insert(next);
                        allowed_succeeding.insert(current, set);
                    }
                }
            }
        }
        /*
            Then, we need to extand our rule set.
            First of, while we are making progress and extending the rule set, keep doing so.
        */
        let mut progress_was_made = true;
        while progress_was_made {
            progress_was_made = false;

            /* Create the next set for allowed succeeding, to prevent updating the current set. */
            let mut next_allowed_succeeding = allowed_succeeding.clone();

            /* Iterate through all rules */
            for rule in fuse_rules.values() {
                /*
                    2)  If a given token B can be after a token A, it means that for each rule that produce such token B,
                        the first token of that rule can also be after A.
                        For example, if B can be after A, and there is C, D -> B, it means that the sequence A, C, D may be valid,
                        as we may merge it into A, B, and B is allowed after A.
                        Hence, C is also allowed after A (note that D is not, only the first token of the rule is allowed).

                    So here, for all allowed tokens set (possible_nexts) containing the result of the current rule,
                    we add it the first token of the rule.
                */
                for possible_nexts in next_allowed_succeeding.values_mut() {
                    if possible_nexts.contains(&rule.result) {
                        progress_was_made |= possible_nexts.insert(usize::from(*rule.state.ids.first().unwrap()));
                    }
                }
                /*
                    3) If a given rule produce a given token A, the last token of this rule can be followed by any tokens that can follow A.
                        For example, if B, C -> A, the token C can be followed by any token that can follow A, as C might be merged into A.

                    So here, for each allowed (posible_nexts) set after a given token (node), if the rule produces that node,
                    we add to the set of the allowed next all the allowed nexts for the last elem of the rule.
                */
                for (node, possible_nexts) in next_allowed_succeeding.iter_mut() {
                    if rule.result == *node {
                        match allowed_succeeding.get(&usize::from(*rule.state.ids.last().unwrap())) {
                            Some(nexts_from_rule) => {
                                for next_from_rule in nexts_from_rule.iter() {
                                    progress_was_made |= possible_nexts.insert(*next_from_rule)
                                }
                            }
                            None => {}
                        }
                    }
                }
            }

            /* Swap the newly computed set with the current one */
            std::mem::swap(&mut allowed_succeeding, &mut next_allowed_succeeding);
        }

        Ok(Self {
            max_rule_size,
            fuse_rules,
            allowed_succeeding,
        })
    }

    pub fn fuse(&self, tokens: &[ParserNode]) -> Option<ParserNode> {
        let state_id = StateId::new(tokens);
        let rule = self.fuse_rules.get(&state_id)?;
        (rule.conversion_func)(tokens)
    }

    pub fn can_succeed(&self, token: &ParserNode, next: &ParserNode) -> bool {
        match self.allowed_succeeding.get(&token.id()) {
            Some(set) => set.contains(&next.id()),
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuleMapCreationError {
    DuplicateRule {
        rule1_loc: ParserRuleDeclarationLocation,
        rule2_loc: ParserRuleDeclarationLocation,
    },
}

impl std::fmt::Display for RuleMapCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateRule { rule1_loc, rule2_loc } => write!(
                f,
                "Duplicate rule found! Rule created at {} clashes with previous rule created at {}",
                rule1_loc, rule2_loc
            ),
        }
    }
}

impl std::error::Error for RuleMapCreationError {}
