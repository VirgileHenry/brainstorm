#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Color {
    Black,
    Blue,
    Colorless,
    Green,
    Red,
    White,
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "B" | "black" => Ok(Self::Black),
            "u" | "U" | "blue" => Ok(Self::Blue),
            "c" | "C" | "colorless" => Ok(Self::Colorless),
            "g" | "G" | "green" => Ok(Self::Green),
            "r" | "R" | "red" => Ok(Self::Red),
            "w" | "W" | "white" => Ok(Self::White),
            other => Err(format!("Unknown Color: {}", other.to_string())),
        }
    }
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::Blue => "blue",
            Self::Colorless => "colorless",
            Self::Green => "green",
            Self::Red => "red",
            Self::White => "white",
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Black => 'b',
            Self::Blue => 'u',
            Self::Colorless => 'c',
            Self::Green => 'g',
            Self::Red => 'r',
            Self::White => 'w',
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Color {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Black,
            Self::Blue,
            Self::Colorless,
            Self::Green,
            Self::Red,
            Self::White,
        ]
        .into_iter()
    }
}
