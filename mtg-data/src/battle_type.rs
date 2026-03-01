#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum BattleType {
    Siege,
}

impl std::str::FromStr for BattleType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "siege" => Ok(Self::Siege),
            _ => Err(crate::ParsingError {
                item: "BattleType",
                message: "provided source does not match",
            }),
        }
    }
}

impl BattleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Siege => "siege",
        }
    }
}

impl std::fmt::Display for BattleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl BattleType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Siege,
        ].into_iter()
    }
}
