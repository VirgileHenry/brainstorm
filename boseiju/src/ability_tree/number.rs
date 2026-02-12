#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Number {
    Number { num: u32 },
    X { x_value: () },
    OrMore { num: u32 },
    AnyNumber,
    ThatMany,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X { x_value } => write!(f, "x, where x is {x_value:?}"), /* Fixme: display */
            Self::Number { num } => write!(f, "{num}"),
            Self::OrMore { num } => write!(f, "{num} or more"),
            Self::AnyNumber => write!(f, "any number of"),
            Self::ThatMany => write!(f, "that many (reference previous number)"),
        }
    }
}
