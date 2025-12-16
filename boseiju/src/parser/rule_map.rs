mod allowed_successors;
mod error;

use crate::parser::node::ParserNode;
use crate::parser::rules::ALL_RULES;
use crate::parser::rules::ParserRule;
use crate::parser::rules::StateId;
use error::RuleMapCreationError;

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
                let [current, next] = window else { unreachable!() };
                let current = usize::from(*current);
                let next = usize::from(*next);
                allowed_succeeding.entry(current).or_default().insert(next);
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
                        let rule_first_token = usize::from(rule.state.first());
                        progress_was_made |= possible_nexts.insert(rule_first_token);
                    }
                }
                /*
                    3) If a given rule produce a given token A, the last token of this rule can be followed by any tokens that can follow A.
                        For example, if B, C -> A, the token C can be followed by any token that can follow A, as C might be merged into A.

                    So here, for each allowed (posible_nexts) set after a given token (node), if the rule produces that node,
                    we add to the set of the allowed next all the allowed nexts for the last elem of the rule.
                */
                let rule_last_token = usize::from(rule.state.last());
                if let Some(allowed_after_rule_result) = allowed_succeeding.get(&rule.result) {
                    let allowed_after_last_token = next_allowed_succeeding.entry(rule_last_token).or_default();
                    for allowed in allowed_after_rule_result.iter() {
                        progress_was_made |= allowed_after_last_token.insert(*allowed);
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
