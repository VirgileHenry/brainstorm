#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CountSpecifier {
    All,
    AllOthers,
    Target,
}

impl CountSpecifier {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "all" | "each" => Some(Self::All),
            "target" | "targets" => Some(Self::Target),
            "all other" | "each other" => Some(Self::AllOthers),
            _ => None,
        }
    }
}
