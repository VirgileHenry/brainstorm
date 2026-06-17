/// Enum for all the keyword abilties that does not require
/// additionnal text besides the keyword.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandaloneKeywordAction {
    Forage,
    Investigate,
    Learn,
    ManifestDread,
    OpenAnAttraction,
    Planeswalk,
    Populate,
    Proliferate,
    RollToVisitYourAttractions,
    SetInMotion,
    Shuffle,
    TimeTravel,
    VentureIntoTheDungeon,
}

impl StandaloneKeywordAction {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Forage,
            Self::Investigate,
            Self::Learn,
            Self::ManifestDread,
            Self::OpenAnAttraction,
            Self::Planeswalk,
            Self::Populate,
            Self::Proliferate,
            Self::RollToVisitYourAttractions,
            Self::SetInMotion,
            Self::Shuffle,
            Self::TimeTravel,
            Self::VentureIntoTheDungeon,
        ]
        .into_iter()
    }
}

impl std::fmt::Display for StandaloneKeywordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forage => write!(f, "forage"),
            Self::Investigate => write!(f, "investigate"),
            Self::Learn => write!(f, "learn"),
            Self::ManifestDread => write!(f, "manifest dread"),
            Self::OpenAnAttraction => write!(f, "open an attraction"),
            Self::Planeswalk => write!(f, "planeswalk"),
            Self::Populate => write!(f, "populate"),
            Self::Proliferate => write!(f, "proliferate"),
            Self::RollToVisitYourAttractions => write!(f, "roll to visit your attractions"),
            Self::SetInMotion => write!(f, "set in motion"),
            Self::Shuffle => write!(f, "shuffle"),
            Self::TimeTravel => write!(f, "time travel"),
            Self::VentureIntoTheDungeon => write!(f, "venture into the dungeon"),
        }
    }
}

impl From<StandaloneKeywordAction> for mtg_data::KeywordAction {
    fn from(value: StandaloneKeywordAction) -> Self {
        match value {
            StandaloneKeywordAction::Forage => mtg_data::KeywordAction::Forage,
            StandaloneKeywordAction::Investigate => mtg_data::KeywordAction::Investigate,
            StandaloneKeywordAction::Learn => mtg_data::KeywordAction::Learn,
            StandaloneKeywordAction::ManifestDread => mtg_data::KeywordAction::ManifestDread,
            StandaloneKeywordAction::OpenAnAttraction => mtg_data::KeywordAction::OpenAnAttraction,
            StandaloneKeywordAction::Planeswalk => mtg_data::KeywordAction::Planeswalk,
            StandaloneKeywordAction::Populate => mtg_data::KeywordAction::Populate,
            StandaloneKeywordAction::Proliferate => mtg_data::KeywordAction::Proliferate,
            StandaloneKeywordAction::RollToVisitYourAttractions => mtg_data::KeywordAction::RollToVisitYourAttractions,
            StandaloneKeywordAction::SetInMotion => mtg_data::KeywordAction::SetInMotion,
            StandaloneKeywordAction::Shuffle => mtg_data::KeywordAction::Shuffle,
            StandaloneKeywordAction::TimeTravel => mtg_data::KeywordAction::TimeTravel,
            StandaloneKeywordAction::VentureIntoTheDungeon => mtg_data::KeywordAction::VentureIntoTheDungeon,
        }
    }
}
