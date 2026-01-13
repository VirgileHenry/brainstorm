#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerProperties {
    HandSize,
    LifeTotal,
    MaximumHandSize,
    StartingLifeTotal,
    OpeningHand,
}

impl PlayerProperties {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "hand size" => Some(Self::HandSize),
            "life total" => Some(Self::LifeTotal),
            "maxmimum hand size" => Some(Self::MaximumHandSize),
            "starting life total" => Some(Self::StartingLifeTotal),
            "opening hand" => Some(Self::OpeningHand),
            _ => None,
        }
    }
}
