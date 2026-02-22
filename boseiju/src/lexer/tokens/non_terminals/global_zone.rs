#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalZone {
    TheBattlefield,
}

impl GlobalZone {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "the battlefield" => Some(Self::TheBattlefield),
            _ => None,
        }
    }
}
