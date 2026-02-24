#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnderControl {
    UnderItsOwnersControl,
    UnderYourControl,
}

impl UnderControl {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "under its owner's control" => Some(Self::UnderItsOwnersControl),
            "under your control" => Some(Self::UnderYourControl),
            _ => None,
        }
    }
}
