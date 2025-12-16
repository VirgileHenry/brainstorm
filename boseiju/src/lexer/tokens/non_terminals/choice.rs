#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Choice {
    Color,
}

impl Choice {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "choose a color" => Some(Self::Color),
            _ => None,
        }
    }
}
