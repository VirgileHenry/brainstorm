#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThisTurn;

impl ThisTurn {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "this turn" => Some(ThisTurn),
            _ => None,
        }
    }
}
