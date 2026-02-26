/// Fixme: what's this ? we can do better
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionKeyword {
    Deals,
    Gain,
    Get,
    Put,
    Reveal,
}

impl ActionKeyword {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "deal" | "deals" | "dealt" => Some(ActionKeyword::Deals),
            "gain" | "gains" => Some(ActionKeyword::Gain),
            "get" | "gets" => Some(ActionKeyword::Get),
            "put" | "puts" => Some(ActionKeyword::Put),
            "reveal" | "reveals" => Some(ActionKeyword::Reveal),
            _ => None,
        }
    }
}
