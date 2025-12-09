#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfReferencing;

impl SelfReferencing {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "this creature" => Some(SelfReferencing),
            "this spell" => Some(SelfReferencing),
            "this card" => Some(SelfReferencing),
            "this token" => Some(SelfReferencing),
            "~" => Some(SelfReferencing),
            _ => None,
        }
    }
}
