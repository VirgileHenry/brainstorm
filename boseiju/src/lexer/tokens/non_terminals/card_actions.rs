#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CardActions {
    Blocks,
    Dies,
    Enters,
    Fight,
    Leave,
}

impl std::fmt::Display for CardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardActions::Blocks => write!(f, "blocks"),
            CardActions::Dies => write!(f, "dies"),
            CardActions::Enters => write!(f, "enters"),
            CardActions::Fight => write!(f, "fights"),
            CardActions::Leave => write!(f, "leaves"),
        }
    }
}

impl CardActions {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "block" | "blocks" => Some(CardActions::Blocks),
            "die" | "dies" => Some(CardActions::Dies),
            "enter" | "enters" => Some(CardActions::Enters),
            "fight" | "fights" => Some(CardActions::Fight),
            "leave" | "leaves" => Some(CardActions::Leave),
            _ => None,
        }
    }
}
