mod error;

use crate::parser::rules::ParserRule;
use error::RuleMapCreationError;

/// A given map of rules to merge tokens.
pub struct RuleMap {
    rules: Vec<ParserRule>,
    merged_to_rules: std::collections::HashMap<usize, Vec<usize>>,
}

impl RuleMap {
    pub fn default() -> Result<Self, RuleMapCreationError> {
        Self::create(crate::parser::rules::default_rules())
    }

    fn create<I: Iterator<Item = ParserRule>>(rules: I) -> Result<Self, RuleMapCreationError> {
        /* Create first the full rule map */
        let rules = rules.into_iter().collect::<Vec<_>>();

        /* Accessors maps */
        let mut lhs_to_rule: std::collections::HashMap<_, _> = std::collections::HashMap::new();
        let mut rhs_to_rule: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();

        for (rule_index, rule) in rules.iter().enumerate() {
            if let Some(prev) = lhs_to_rule.insert(rule.expanded, rule_index) {
                return Err(RuleMapCreationError::DuplicateRule {
                    rule1_loc: rules[prev].creation_loc.clone(),
                    rule2_loc: rule.creation_loc.clone(),
                });
            }
            rhs_to_rule.entry(rule.merged).or_default().push(rule_index);
        }

        Ok(Self {
            rules,
            merged_to_rules: rhs_to_rule,
        })
    }

    pub fn get_rules_for_token(&self, node_id: usize) -> Vec<&ParserRule> {
        let rule_indices = match self.merged_to_rules.get(&node_id) {
            Some(indices) => indices,
            None => return Vec::with_capacity(0),
        };

        rule_indices.iter().map(|rule_index| &self.rules[*rule_index]).collect()
    }
}
