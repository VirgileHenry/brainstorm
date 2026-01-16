#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndirectReference {
    ItsController,
    ItsOwnersHand,
    ItsOwnersLibrary,
    Owner,
}

impl IndirectReference {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "its controller" => Some(Self::ItsController),
            "its owner's hand" => Some(Self::ItsOwnersHand),
            "its owner's library" => Some(Self::ItsOwnersLibrary),
            "'s owner" => Some(Self::Owner),
            _ => None,
        }
    }
}
