#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mana {
    X,
    Any(usize),
    Colored(crate::Color),
    Hybrid(crate::Color, crate::Color),
    MonocoloredHybrid(usize, crate::Color),
    Phyrexian(crate::Color),
    HybridPhyrexian(crate::Color, crate::Color),
    Snow,
}

impl std::str::FromStr for Mana {
    type Err = String;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        if from.starts_with('{') && from.ends_with('}') {
            let symbols = from[1..from.len() - 1]
                .split('/')
                .map(ManaSymbol::parse_symbol)
                .collect::<Result<Vec<_>, _>>()?;
            match symbols.as_slice() {
                [ManaSymbol::X] => Ok(Mana::X),
                [ManaSymbol::Any(num)] => Ok(Mana::Any(*num)),
                [ManaSymbol::Colored(color)] => Ok(Mana::Colored(*color)),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2)] => Ok(Mana::Hybrid(*c1, *c2)),
                [ManaSymbol::Any(num), ManaSymbol::Colored(color)] => Ok(Mana::MonocoloredHybrid(*num, *color)),
                [ManaSymbol::Colored(color), ManaSymbol::Phyrexian] => Ok(Mana::Phyrexian(*color)),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2), ManaSymbol::Phyrexian] => Ok(Mana::HybridPhyrexian(*c1, *c2)),
                [ManaSymbol::Snow] => Ok(Mana::Snow),
                _ => Err(format!("Invalid symbol combination: {symbols:?}")),
            }
        } else {
            Err(format!("Mana cost shall be between curly braces, got {from}"))
        }
    }
}

impl Mana {
    pub const VARIANT_COUNT: usize = 8;

    pub fn id(&self) -> u32 {
        match self {
            Mana::X => 0,
            Mana::Snow => 1,
            Mana::Any(_) => 2,
            Mana::Colored(_) => 3,
            Mana::Hybrid(_, _) => 4,
            Mana::MonocoloredHybrid(_, _) => 5,
            Mana::Phyrexian(_) => 6,
            Mana::HybridPhyrexian(_, _) => 7,
        }
    }
}

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mana::X => write!(f, "{{x}}"),
            Mana::Snow => write!(f, "{{s}}"),
            Mana::Any(n) => write!(f, "{{{n}}}"),
            Mana::Colored(c) => write!(f, "{{{}}}", c.as_char()),
            Mana::Hybrid(c1, c2) => write!(f, "{{{}/{}}}", c1.as_char(), c2.as_char()),
            Mana::MonocoloredHybrid(n, c) => write!(f, "{{{n}/{}}}", c.as_char()),
            Mana::Phyrexian(c) => write!(f, "{{{}/p}}", c.as_char()),
            Mana::HybridPhyrexian(c1, c2) => write!(f, "{{{}/{}/p}}", c1.as_char(), c2.as_char()),
        }
    }
}

/// Inner type used to parse mana costs.
enum ManaSymbol {
    X,
    Any(usize),
    Colored(crate::Color),
    Phyrexian,
    Snow,
}

impl std::fmt::Debug for ManaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManaSymbol::X => write!(f, "x"),
            ManaSymbol::Any(num) => write!(f, "{num}"),
            ManaSymbol::Colored(color) => write!(f, "{}", color.as_char()),
            ManaSymbol::Phyrexian => write!(f, "p"),
            ManaSymbol::Snow => write!(f, "s"),
        }
    }
}

impl ManaSymbol {
    fn parse_symbol(input: &str) -> Result<ManaSymbol, String> {
        return match input {
            "w" | "W" => Ok(ManaSymbol::Colored(crate::Color::White)),
            "b" | "B" => Ok(ManaSymbol::Colored(crate::Color::Black)),
            "r" | "R" => Ok(ManaSymbol::Colored(crate::Color::Red)),
            "u" | "U" => Ok(ManaSymbol::Colored(crate::Color::Blue)),
            "g" | "G" => Ok(ManaSymbol::Colored(crate::Color::Green)),
            "c" | "C" => Ok(ManaSymbol::Colored(crate::Color::Colorless)),
            "x" | "X" => Ok(ManaSymbol::X),
            "s" | "S" => Ok(ManaSymbol::Snow),
            "p" | "P" => Ok(ManaSymbol::Phyrexian),
            other => match other.parse() {
                Ok(num) => Ok(ManaSymbol::Any(num)),
                Err(_) => Err(format!("Unknown mana symbol: {other}")),
            },
        };
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::Color;

    use super::*;

    #[test]
    fn test_mana_cost_parsing() {
        assert_eq!(Mana::from_str("{r}"), Ok(Mana::Colored(Color::Red)));
        assert_eq!(Mana::from_str("{x}"), Ok(Mana::X));
        assert_eq!(Mana::from_str("{12}"), Ok(Mana::Any(12)));
        assert_eq!(Mana::from_str("{s}"), Ok(Mana::Snow));
        assert_eq!(Mana::from_str("{r/g}"), Ok(Mana::Hybrid(Color::Red, Color::Green)));
        assert_eq!(Mana::from_str("{r/p}"), Ok(Mana::Phyrexian(Color::Red)));
        assert_eq!(Mana::from_str("{3/r}"), Ok(Mana::MonocoloredHybrid(3, Color::Red)));
        assert_eq!(Mana::from_str("{r/g/p}"), Ok(Mana::HybridPhyrexian(Color::Red, Color::Green)));
    }
}
