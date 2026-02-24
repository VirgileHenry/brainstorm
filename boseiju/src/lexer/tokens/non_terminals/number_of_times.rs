#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberOfTimes {
    FirstTime,
    SecondTime,
    ThirdTime,
}

impl NumberOfTimes {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "first time" => Some(Self::FirstTime),
            "second time" => Some(Self::SecondTime),
            "third time" => Some(Self::ThirdTime),
            _ => None,
        }
    }
}
