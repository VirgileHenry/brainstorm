#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinLoseClause {
    WinTheGame,
    LoseTheGame,
}

impl WinLoseClause {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "win the game" => Some(Self::WinTheGame),
            "lose the game" => Some(Self::LoseTheGame),
            _ => None,
        }
    }
}
