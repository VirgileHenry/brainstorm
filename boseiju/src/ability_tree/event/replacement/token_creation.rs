#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenCreation {
    pub amount: crate::ability_tree::number::Number,
    pub token: ReplacedTokenKind,
}

impl crate::ability_tree::AbilityTreeImpl for TokenCreation {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "create token:")?;
        out.push_inter_branch()?;
        write!(out, "amount: {}", self.amount)?;
        out.next_final_branch()?;
        write!(out, "of token: {}", self.token)?;
        out.pop_branch();
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ReplacedTokenKind {
    PreviouslyMentionnedToken,
    NewToken(crate::card::layout::TokenLayout),
}

impl std::fmt::Display for ReplacedTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreviouslyMentionnedToken => write!(f, "previously mentionned token"),
            Self::NewToken(token) => write!(f, "{}", token.name),
        }
    }
}
