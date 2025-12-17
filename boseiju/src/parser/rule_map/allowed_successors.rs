use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct AllowedSuccessors {
    #[allow(unused)]
    from_node: usize,
    allowed_successors: HashSet<usize>,
}

impl AllowedSuccessors {
    pub fn new(from_node: usize) -> Self {
        Self {
            from_node,
            allowed_successors: HashSet::new(),
        }
    }

    pub fn allowed(&self, token: usize) -> bool {
        self.allowed_successors.contains(&token)
    }

    /// Simply allow the given token.
    /// This puts it straight into the allowed successors.
    pub fn allow_next(&mut self, next: usize) -> bool {
        self.allowed_successors.insert(next)
    }

    /// If the result of the given rule is in our set of allowed successors,
    /// also put the first token of the rule in our allowed successors.
    ///
    /// Since the new token might use that rule to merge into a result token we allowed,
    /// we might as well allow this new token.
    ///
    /// For example, if `B` is allowed after `A`, and there is a rule that merges `C, D` into `B`,
    /// `C` is allowed after `A`, since we might have a legal merge `A, C, D -> A, B`.
    ///
    /// This functions also return wheteher the set was updated or not.
    pub fn allow_rule_first_token_from_result(&mut self, rule: &crate::parser::rules::ParserRule) -> bool {
        if self.allowed_successors.contains(&rule.result) {
            let rule_first_token = usize::from(rule.state.first());
            self.allowed_successors.insert(rule_first_token)
        } else {
            false
        }
    }

    /// For a given rule `A, B -> C`, all tokens that are allowed after the result (`C`) shall also be allowed
    /// after the last token of that rule (`B`).
    ///
    /// For instance, with the previous rule `A, B -> C`,
    /// if we have `A, B, D` and `C, D` is allowed, then `B, C` shall be allowed as the `B` might be merged into `C`.
    /// Therefore, the aforementionned sequence shall be valid.
    pub fn allow_allowed_after_rule_result_from_last_token(&mut self, allowed_after_result: &Self) -> bool {
        let mut updated = false;
        for allowed in allowed_after_result.allowed_successors.iter() {
            updated |= self.allowed_successors.insert(*allowed);
        }
        updated
    }
}
