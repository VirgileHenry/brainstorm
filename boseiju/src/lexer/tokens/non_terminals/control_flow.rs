#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlFlow {
    NewLine,
    Comma,
    Dot,
    Colons,
    LongDash,
    Bullet,
}

impl ControlFlow {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "\n" => Some(ControlFlow::NewLine),
            "," => Some(ControlFlow::Comma),
            "." => Some(ControlFlow::Dot),
            ":" => Some(ControlFlow::Colons),
            "—" => Some(ControlFlow::LongDash),
            "•" => Some(ControlFlow::Bullet),
            _ => None,
        }
    }
}
