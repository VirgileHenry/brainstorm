#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntoHand {
    IntoHisOwnersHand,
}

impl IntoHand {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "into its owner's hand" => Some(Self::IntoHisOwnersHand),
            _ => None,
        }
    }
}
