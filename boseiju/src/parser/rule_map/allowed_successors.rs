use crate::parser::rules::ParserRule;
use std::collections::HashSet;

pub struct AllowedSuccessors {
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

    pub fn update_for_rule(&mut self, rule: ParserRule) -> bool {
        let mut update_made = false;

        /* Iterate over the rules raw tokens. If we found our from_node, add the next node in the allowed successors. */
        for window in rule.state.ids.windows(2).take(rule.state.size.saturating_sub(1)) {
            let [current, next] = window else { unreachable!() };
            let current = usize::from(*current);
            let next = usize::from(*next);
            if current == self.from_node {
                update_made |= self.allowed_successors.insert(next);
            }
        }

        /* If the rule result is in our allowed successors, the first token of the rule shall also be */
        if self.allowed_successors.contains(&rule.result) {
            let rule_first_token = usize::from(rule.state.first());
            update_made |= self.allowed_successors.insert(rule_first_token);
        }

        update_made
    }
}
