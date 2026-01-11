#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonKind {
    NonCreature,
    NonLand,
    NonToken,
}

impl NonKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "noncreature" => Some(Self::NonCreature),
            "nonland" => Some(Self::NonLand),
            "nontoken" => Some(Self::NonToken),
            _ => None,
        }
    }
}
