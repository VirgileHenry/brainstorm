#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedToken {
    KomasCoil,
}

impl std::fmt::Display for NamedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KomasCoil => write!(f, "koma's coil"),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for NamedToken {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "koma's coil" => Some(Self::KomasCoil),
            "~'s coil" => Some(Self::KomasCoil), /* Weird case for koma creating koma's coil */
            _ => None,
        }
    }
}
