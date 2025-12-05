#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotOfAKind;

impl NotOfAKind {
    pub const COUNT: usize = 1;
    pub const fn id(&self) -> usize {
        0
    }

    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "non-" => Some(NotOfAKind),
            _ => None,
        }
    }
}
