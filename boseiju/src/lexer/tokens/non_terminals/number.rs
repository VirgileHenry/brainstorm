#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    AnyNumber,
    Number { num: u32 },
    NumberOf,
    OrMore { num: u32 },
    ThatMany,
    TwiceThatMany,
    UpTo { num: u32 },
    X,
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
        } else if let Some(stripped) = source.strip_prefix("up to ") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Self::UpTo { num })
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
