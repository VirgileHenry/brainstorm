#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Mana {
    X,
    Any(AnyMana),
    Colored(ColoredMana),
    Hybrid(HybridMana),
    MonocoloredHybrid(MonocoloredHybridMana),
    Phyrexian(PhyrexianMana),
    HybridPhyrexian(HybridPhyrexianMana),
    Snow,
}

impl Mana {
    pub fn mana_value(&self) -> usize {
        match self {
            Mana::X => 0,
            Mana::Any(any_mana) => any_mana.number,
            Mana::Colored(_) => 1,
            Mana::Hybrid(_) => 1,
            Mana::MonocoloredHybrid(hybrid_mana) => 1.max(hybrid_mana.number),
            Mana::Phyrexian(_) => 1,
            Mana::HybridPhyrexian(_) => 1,
            Mana::Snow => 1,
        }
    }
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
                [ManaSymbol::Any(number)] => Ok(Mana::Any(AnyMana { number: *number })),
                [ManaSymbol::Colored(color)] => Ok(Mana::Colored(ColoredMana { color: *color })),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2)] => Ok(Mana::Hybrid(HybridMana {
                    color_1: *c1,
                    color_2: *c2,
                })),
                [ManaSymbol::Any(number), ManaSymbol::Colored(color)] => Ok(Mana::MonocoloredHybrid(MonocoloredHybridMana {
                    number: *number,
                    color: *color,
                })),
                [ManaSymbol::Colored(color), ManaSymbol::Phyrexian] => Ok(Mana::Phyrexian(PhyrexianMana { color: *color })),
                [ManaSymbol::Colored(c1), ManaSymbol::Colored(c2), ManaSymbol::Phyrexian] => {
                    Ok(Mana::HybridPhyrexian(HybridPhyrexianMana {
                        color_1: *c1,
                        color_2: *c2,
                    }))
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
            Mana::Any(mana) => write!(f, "{mana}"),
            Mana::Colored(mana) => write!(f, "{mana}"),
            Mana::Hybrid(mana) => write!(f, "{mana}"),
            Mana::MonocoloredHybrid(mana) => write!(f, "{mana}"),
            Mana::Phyrexian(mana) => write!(f, "{mana}"),
            Mana::HybridPhyrexian(mana) => write!(f, "{mana}"),
        }
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct AnyMana {
    pub number: usize,
}

impl idris::Idris<usize> for AnyMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for AnyMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.number)
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ColoredMana {
    pub color: crate::Color,
}

impl idris::Idris<usize> for ColoredMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for ColoredMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.color.as_char())
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct HybridMana {
    pub color_1: crate::Color,
    pub color_2: crate::Color,
}

impl idris::Idris<usize> for HybridMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for HybridMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}/{}}}", self.color_1.as_char(), self.color_2.as_char())
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct MonocoloredHybridMana {
    pub color: crate::Color,
    pub number: usize,
}

impl idris::Idris<usize> for MonocoloredHybridMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for MonocoloredHybridMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}/{}}}", self.number, self.color.as_char())
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PhyrexianMana {
    pub color: crate::Color,
}

impl idris::Idris<usize> for PhyrexianMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for PhyrexianMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}/p}}", self.color.as_char())
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct HybridPhyrexianMana {
    pub color_1: crate::Color,
    pub color_2: crate::Color,
}

impl idris::Idris<usize> for HybridPhyrexianMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

impl std::fmt::Display for HybridPhyrexianMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}/{}/p}}", self.color_1.as_char(), self.color_2.as_char())
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
