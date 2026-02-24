#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChoiceReference {
    ChosenColor,
    HasntBeenChosen,
}

impl ChoiceReference {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "chosen color" => Some(Self::ChosenColor),
            "hasn't been chosen" => Some(Self::HasntBeenChosen),
            _ => None,
        }
    }
}
