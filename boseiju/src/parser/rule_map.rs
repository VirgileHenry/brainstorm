mod allowed_successors;
mod error;

use crate::parser::node::ParserNode;
use crate::parser::rules::ALL_RULES;
use crate::parser::rules::ParserRule;
use crate::parser::rules::StateId;
use error::RuleMapCreationError;
use std::collections::HashMap;

/// A given map of rules to merge tokens.
pub struct RuleMap {
    /// Number of tokens in the rule wth the biggest number of tokens.
    /// This allow to avoid calling fuse unnecessarily on windows that are too large.
    max_rule_size: usize,
    /// List of the rules we have for merging.
    /// For each state id (list of tokens), there is a rule that merge these tokens.
    /// Note that the rule may fail to merge the tokens,
    ///
    /// and the presence of a rule in the HashMap for a given state id does not guarantee
    /// that the tokens will be fused.
    rules: HashMap<StateId, ParserRule>,
    /// For each token id, we store the list of tokens that can be found after it,
    /// following our rules.
    /// This allows to quickly find that our state will fail,
    /// without having to explore it entirely.
    allowed_succeeding: HashMap<usize, allowed_successors::AllowedSuccessors>,
}

impl RuleMap {
    pub fn default() -> Result<Self, RuleMapCreationError> {
        Self::create(ALL_RULES)
    }

    fn create(rules: &[&[ParserRule]]) -> Result<Self, RuleMapCreationError> {
        /* Create first the full rule map */
        let flat_rules = rules.iter().cloned().flatten().cloned().collect();
        let rules = create_rules_map(flat_rules)?;

        /* The rule map can be used to get the max rule size */
        let max_rule_size = rules.keys().map(|k| k.size).max().unwrap_or(0);

        /* And we can build the token succession rules */
        let allowed_succeeding = create_allow_succeeding(&rules);

        Ok(Self {
            max_rule_size,
            rules,
            allowed_succeeding,
        })
    }

    pub fn max_rule_size(&self) -> usize {
        self.max_rule_size
    }

    pub fn fuse(&self, tokens: &[ParserNode]) -> Option<ParserNode> {
        let state_id = StateId::new(tokens);
        let rule = self.rules.get(&state_id)?;
        (rule.conversion_func)(tokens)
    }

    pub fn can_succeed(&self, token: &ParserNode, next: &ParserNode) -> bool {
        match self.allowed_succeeding.get(&token.id()) {
            Some(set) => set.allowed(next.id()),
            None => false,
        }
    }
}

/// Create a rule map from the flat rules vector.
///
/// If two rules require the same input state, this will return a rule map creation error.
///
/// Otherwise, for each state id, we will have the corresponding rule to merge that state.
fn create_rules_map(rules: Vec<ParserRule>) -> Result<HashMap<StateId, ParserRule>, RuleMapCreationError> {
    let mut fuse_rules: HashMap<StateId, ParserRule> = HashMap::new();
    for rule in rules.into_iter() {
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
    Ok(fuse_rules)
}

/// Find which tokens are allowed to be next to each other.
///
/// This function will take in a set of rules, and will create a map, where each token is mapped
/// to the tokens that can be found after it.
///
/// This allow to find errors early in parser states, and not spend time exploring nodes that are doomed to fail.
///
/// We say that two tokens are "allowed" if they can be next to each other in a parser state.
/// If two tokens are not allowed, it means that the state will fail as no rules will ever merge them.
///
/// The core idea is to loop over all the rules, and start by saying that tokens next to each others in a rule are allowed.
/// Then we extend this set by checking tokens that might get merged into allowed tokens.
fn create_allow_succeeding(rules: &HashMap<StateId, ParserRule>) -> HashMap<usize, allowed_successors::AllowedSuccessors> {
    let mut allowed_succeeding: HashMap<usize, allowed_successors::AllowedSuccessors> = HashMap::new();

    /* Start by iterating through all the rules, as tokens next to each others in a rule are allowed */
    for state in rules.keys() {
        /* For each rule, iterate through succeeding tokens, and set the follower to be allowed to follow the followee. */
        for window in state.ids.windows(2).take(state.size.saturating_sub(1)) {
            let [current, next] = window else { unreachable!() };
            let current = usize::from(*current);
            let next = usize::from(*next);
            allowed_succeeding
                .entry(current)
                .or_insert(allowed_successors::AllowedSuccessors::new(current))
                .allow_next(next);
        }
    }

    /* Then, we need to extend our rule set. */
    let mut updated = true;
    /* While we are extending, keep extending, until no changes are made */
    while updated {
        updated = false;

        /* Create the next set for allowed succeeding, to prevent updating the current set. */
        let mut next_allowed_succeeding = allowed_succeeding.clone();

        /* Iterate through all rules */
        for rule in rules.values() {
            /* 1) Add allowed from rule result / first token */
            for possible_nexts in next_allowed_succeeding.values_mut() {
                updated |= possible_nexts.allow_rule_first_token_from_result(rule);
            }
            /* 2) Add allowed from rule last token / result */
            let rule_last_token = usize::from(rule.state.last());
            if let Some(allowed_after_result) = allowed_succeeding.get(&rule.result) {
                let allowed_after_last_token = next_allowed_succeeding
                    .entry(rule_last_token)
                    .or_insert(allowed_successors::AllowedSuccessors::new(rule_last_token));
                updated |= allowed_after_last_token.allow_allowed_after_rule_result_from_last_token(allowed_after_result);
            }
        }

        /* Swap the newly computed set with the current one */
        std::mem::swap(&mut allowed_succeeding, &mut next_allowed_succeeding);
    }

    allowed_succeeding
}
