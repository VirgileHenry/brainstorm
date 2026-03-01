#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PowerToughnessModElements {
    Plus,
    Minus,
    Bar,
}

impl PowerToughnessModElements {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "+" => Some(PowerToughnessModElements::Plus),
            "-" => Some(PowerToughnessModElements::Minus),
            "/" => Some(PowerToughnessModElements::Bar),
            _ => None,
        }
    }
}
