#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ManaSymbol {
    X,
    Any(AnyMana),
    Colored(ColoredMana),
    Hybrid(HybridMana),
    MonocoloredHybrid(MonocoloredHybridMana),
    Phyrexian(PhyrexianMana),
    HybridPhyrexian(HybridPhyrexianMana),
    Snow,
}

impl ManaSymbol {
    pub fn mana_value(&self) -> usize {
        match self {
            ManaSymbol::X => 0,
            ManaSymbol::Any(any_mana) => any_mana.number,
            ManaSymbol::Colored(_) => 1,
            ManaSymbol::Hybrid(_) => 1,
            ManaSymbol::MonocoloredHybrid(hybrid_mana) => 1.max(hybrid_mana.number),
            ManaSymbol::Phyrexian(_) => 1,
            ManaSymbol::HybridPhyrexian(_) => 1,
            ManaSymbol::Snow => 1,
        }
    }
}

impl std::str::FromStr for ManaSymbol {
    type Err = String;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        if from.starts_with('{') && from.ends_with('}') {
            let symbols = from[1..from.len() - 1]
                .split('/')
                .map(ManaSingleSymbol::parse_symbol)
                .collect::<Result<Vec<_>, _>>()?;
            match symbols.as_slice() {
                [ManaSingleSymbol::X] => Ok(ManaSymbol::X),
                [ManaSingleSymbol::Any(number)] => Ok(ManaSymbol::Any(AnyMana { number: *number })),
                [ManaSingleSymbol::Colored(color)] => Ok(ManaSymbol::Colored(ColoredMana { color: *color })),
                [ManaSingleSymbol::Colored(c1), ManaSingleSymbol::Colored(c2)] => Ok(ManaSymbol::Hybrid(HybridMana {
                    color_1: *c1,
                    color_2: *c2,
                })),
                [ManaSingleSymbol::Any(number), ManaSingleSymbol::Colored(color)] => {
                    Ok(ManaSymbol::MonocoloredHybrid(MonocoloredHybridMana {
                        number: *number,
                        color: *color,
                    }))
                }
                [ManaSingleSymbol::Colored(color), ManaSingleSymbol::Phyrexian] => {
                    Ok(ManaSymbol::Phyrexian(PhyrexianMana { color: *color }))
                }
                [
                    ManaSingleSymbol::Colored(c1),
                    ManaSingleSymbol::Colored(c2),
                    ManaSingleSymbol::Phyrexian,
                ] => Ok(ManaSymbol::HybridPhyrexian(HybridPhyrexianMana {
                    color_1: *c1,
                    color_2: *c2,
                })),
                [ManaSingleSymbol::Snow] => Ok(ManaSymbol::Snow),
                _ => Err(format!("Invalid symbol combination: {symbols:?}")),
            }
        } else {
            Err(format!("Mana cost shall be between curly braces, got {from}"))
        }
    }
}

impl std::fmt::Display for ManaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManaSymbol::X => write!(f, "{{x}}"),
            ManaSymbol::Snow => write!(f, "{{s}}"),
            ManaSymbol::Any(mana) => write!(f, "{mana}"),
            ManaSymbol::Colored(mana) => write!(f, "{mana}"),
            ManaSymbol::Hybrid(mana) => write!(f, "{mana}"),
            ManaSymbol::MonocoloredHybrid(mana) => write!(f, "{mana}"),
            ManaSymbol::Phyrexian(mana) => write!(f, "{mana}"),
            ManaSymbol::HybridPhyrexian(mana) => write!(f, "{mana}"),
        }
    }
}

/// A mana symbol with a number on it, representing a fixed amount of any kind of mana.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnyMana {
    pub number: usize,
}

impl idris::Idris for AnyMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{number}"
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
pub struct ColoredMana {
    pub color: crate::Color,
}

impl idris::Idris for ColoredMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{color}"
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
pub struct HybridMana {
    pub color_1: crate::Color,
    pub color_2: crate::Color,
}

impl idris::Idris for HybridMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{color/color}"
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
pub struct MonocoloredHybridMana {
    pub color: crate::Color,
    pub number: usize,
}

impl idris::Idris for MonocoloredHybridMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{number/color}"
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
pub struct PhyrexianMana {
    pub color: crate::Color,
}

impl idris::Idris for PhyrexianMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{color/p}"
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
pub struct HybridPhyrexianMana {
    pub color_1: crate::Color,
    pub color_2: crate::Color,
}

impl idris::Idris for HybridPhyrexianMana {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "{color/color/p}"
    }
}

impl std::fmt::Display for HybridPhyrexianMana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}/{}/p}}", self.color_1.as_char(), self.color_2.as_char())
    }
}

/// Inner type used to parse mana costs.
enum ManaSingleSymbol {
    X,
    Any(usize),
    Colored(crate::Color),
    Phyrexian,
    Snow,
}

impl std::fmt::Debug for ManaSingleSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManaSingleSymbol::X => write!(f, "x"),
            ManaSingleSymbol::Any(num) => write!(f, "{num}"),
            ManaSingleSymbol::Colored(color) => write!(f, "{}", color.as_char()),
            ManaSingleSymbol::Phyrexian => write!(f, "p"),
            ManaSingleSymbol::Snow => write!(f, "s"),
        }
    }
}

impl ManaSingleSymbol {
    fn parse_symbol(input: &str) -> Result<ManaSingleSymbol, String> {
        return match input {
            "w" | "W" => Ok(ManaSingleSymbol::Colored(crate::Color::White)),
            "b" | "B" => Ok(ManaSingleSymbol::Colored(crate::Color::Black)),
            "r" | "R" => Ok(ManaSingleSymbol::Colored(crate::Color::Red)),
            "u" | "U" => Ok(ManaSingleSymbol::Colored(crate::Color::Blue)),
            "g" | "G" => Ok(ManaSingleSymbol::Colored(crate::Color::Green)),
            "c" | "C" => Ok(ManaSingleSymbol::Colored(crate::Color::Colorless)),
            "x" | "X" => Ok(ManaSingleSymbol::X),
            "s" | "S" => Ok(ManaSingleSymbol::Snow),
            "p" | "P" => Ok(ManaSingleSymbol::Phyrexian),
            other => match other.parse() {
                Ok(num) => Ok(ManaSingleSymbol::Any(num)),
                Err(_) => Err(format!("Unknown mana symbol: {other}")),
            },
        };
    }
}
