use crate::parser::rules::ParserRuleDeclarationLocation;

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
