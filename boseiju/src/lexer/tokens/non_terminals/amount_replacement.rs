#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmountReplacement {
    TwiceThatMany,
    HalfThatMany,
    ThatManyPlus,
    ThatManyMinus,
}

impl AmountReplacement {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "twice that many" => Some(Self::TwiceThatMany),
            "half that many" => Some(Self::HalfThatMany),
            "that many plus" => Some(Self::ThatManyPlus),
            "that many minus" => Some(Self::ThatManyMinus),
            _ => None,
        }
    }
}
