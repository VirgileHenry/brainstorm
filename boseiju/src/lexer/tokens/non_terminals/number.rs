#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Number { num: u32 },
    X,
    OrMore { num: u32 },
    AnyNumber,
    ThatMany,
    TwiceThatMany,
    NumberOf,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Number { num } => write!(f, "{num}"),
            Self::OrMore { num } => write!(f, "{num} or more"),
            Self::AnyNumber => write!(f, "any number of"),
            Self::ThatMany => write!(f, "that many"),
            Self::NumberOf => write!(f, "number of"),
            Self::TwiceThatMany => write!(f, "twice that many"),
        }
    }
}

impl Number {
    pub fn try_from_str(source: &str) -> Option<Self> {
        if let Some(num) = crate::utils::parse_num(source) {
            Some(Self::Number { num })
        } else if let Some(stripped) = source.strip_suffix(" or more") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::OrMore { num })
        } else if let Some(stripped) = source.strip_suffix(" or greater") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::OrMore { num })
        } else {
            match source {
                "x" => Some(Self::X),
                "any number of" => Some(Self::AnyNumber),
                "that many" => Some(Self::ThatMany),
                "number of" | "amount of" => Some(Self::NumberOf),
                "twice that many" => Some(Self::TwiceThatMany),
                _ => None,
            }
        }
    }
}
