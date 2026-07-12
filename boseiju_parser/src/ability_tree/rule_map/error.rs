#[derive(Debug, Clone)]
pub enum RuleMapCreationError {}

impl std::fmt::Display for RuleMapCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Failed to create rule map !"),
        }
    }
}

impl std::error::Error for RuleMapCreationError {}
