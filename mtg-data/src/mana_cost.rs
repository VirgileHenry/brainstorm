#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mana {
    X,
    Any { number: usize },
    Colored { color: crate::Color },
    Hybrid { c1: crate::Color, c2: crate::Color },
    MonocoloredHybrid { number: usize, color: crate::Color },
    Phyrexian { color: crate::Color },
    HybridPhyrexian { c1: crate::Color, c2: crate::Color },
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
                [ManaSymbol::Any(number)] => Ok(Mana::Any { number: *number }),
                [ManaSymbol::Colored(color)] => Ok(Mana::Colored { color: *color }),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2)] => Ok(Mana::Hybrid { c1: *c1, c2: *c2 }),
                [ManaSymbol::Any(number), ManaSymbol::Colored(color)] => Ok(Mana::MonocoloredHybrid {
                    number: *number,
                    color: *color,
                }),
                [ManaSymbol::Colored(color), ManaSymbol::Phyrexian] => Ok(Mana::Phyrexian { color: *color }),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2), ManaSymbol::Phyrexian] => {
                    Ok(Mana::HybridPhyrexian { c1: *c1, c2: *c2 })
                }
                [ManaSymbol::Snow] => Ok(Mana::Snow),
                _ => Err(format!("Invalid symbol combination: {symbols:?}")),
            }
        } else {
            Err(format!("Mana cost shall be between curly braces, got {from}"))
        }
    }
}

impl std::fmt::Display for Mana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mana::X => write!(f, "{{x}}"),
            Mana::Snow => write!(f, "{{s}}"),
            Mana::Any { number } => write!(f, "{{{number}}}"),
            Mana::Colored { color } => write!(f, "{{{}}}", color.as_char()),
            Mana::Hybrid { c1, c2 } => write!(f, "{{{}/{}}}", c1.as_char(), c2.as_char()),
            Mana::MonocoloredHybrid { number, color } => write!(f, "{{{number}/{}}}", color.as_char()),
            Mana::Phyrexian { color } => write!(f, "{{{}/p}}", color.as_char()),
            Mana::HybridPhyrexian { c1, c2 } => write!(f, "{{{}/{}/p}}", c1.as_char(), c2.as_char()),
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
        assert_eq!(Mana::from_str("{r}"), Ok(Mana::Colored { color: Color::Red }));
        assert_eq!(Mana::from_str("{x}"), Ok(Mana::X));
        assert_eq!(Mana::from_str("{12}"), Ok(Mana::Any { number: 12 }));
        assert_eq!(Mana::from_str("{s}"), Ok(Mana::Snow));
        assert_eq!(
            Mana::from_str("{r/g}"),
            Ok(Mana::Hybrid {
                c1: Color::Red,
                c2: Color::Green
            })
        );
        assert_eq!(Mana::from_str("{r/p}"), Ok(Mana::Phyrexian { color: Color::Red }));
        assert_eq!(
            Mana::from_str("{3/r}"),
            Ok(Mana::MonocoloredHybrid {
                number: 3,
                color: Color::Red
            })
        );
        assert_eq!(
            Mana::from_str("{r/g/p}"),
            Ok(Mana::HybridPhyrexian {
                c1: Color::Red,
                c2: Color::Green
            })
        );
    }
}
