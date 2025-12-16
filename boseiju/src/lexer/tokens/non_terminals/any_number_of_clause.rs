#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyNumberOfClause;

impl AnyNumberOfClause {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "a deck can have any number of cards named ~." => Some(Self),
            _ => None,
        }
    }
}
