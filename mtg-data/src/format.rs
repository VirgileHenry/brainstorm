#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Format {
    Alchemy,
    Brawl,
    Commander,
    Duel,
    Explorer,
    Future,
    Gladiator,
    Historic,
    HistoricBrawl,
    Legacy,
    Modern,
    Oathbreaker,
    Pauper,
    PauperCommander,
    Penny,
    Pionner,
    Predh,
    Premodern,
    Standard,
    Vintage,
}

impl std::str::FromStr for Format {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alchemy" => Ok(Self::Alchemy),
            "brawl" => Ok(Self::Brawl),
            "commander" => Ok(Self::Commander),
            "duel" => Ok(Self::Duel),
            "explorer" => Ok(Self::Explorer),
            "future" => Ok(Self::Future),
            "gladiator" => Ok(Self::Gladiator),
            "historic" => Ok(Self::Historic),
            "historic brawl" => Ok(Self::HistoricBrawl),
            "legacy" => Ok(Self::Legacy),
            "modern" => Ok(Self::Modern),
            "oathbreaker" => Ok(Self::Oathbreaker),
            "pauper" => Ok(Self::Pauper),
            "pauper commander" => Ok(Self::PauperCommander),
            "penny" => Ok(Self::Penny),
            "pionner" => Ok(Self::Pionner),
            "predh" => Ok(Self::Predh),
            "premodern" => Ok(Self::Premodern),
            "standard" => Ok(Self::Standard),
            "vintage" => Ok(Self::Vintage),
            _ => Err(crate::ParsingError {
                item: "Format",
                message: "provided source does not match",
            }),
        }
    }
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alchemy => "alchemy",
            Self::Brawl => "brawl",
            Self::Commander => "commander",
            Self::Duel => "duel",
            Self::Explorer => "explorer",
            Self::Future => "future",
            Self::Gladiator => "gladiator",
            Self::Historic => "historic",
            Self::HistoricBrawl => "historic brawl",
            Self::Legacy => "legacy",
            Self::Modern => "modern",
            Self::Oathbreaker => "oathbreaker",
            Self::Pauper => "pauper",
            Self::PauperCommander => "pauper commander",
            Self::Penny => "penny",
            Self::Pionner => "pionner",
            Self::Predh => "predh",
            Self::Premodern => "premodern",
            Self::Standard => "standard",
            Self::Vintage => "vintage",
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Format {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Alchemy,
            Self::Brawl,
            Self::Commander,
            Self::Duel,
            Self::Explorer,
            Self::Future,
            Self::Gladiator,
            Self::Historic,
            Self::HistoricBrawl,
            Self::Legacy,
            Self::Modern,
            Self::Oathbreaker,
            Self::Pauper,
            Self::PauperCommander,
            Self::Penny,
            Self::Pionner,
            Self::Predh,
            Self::Premodern,
            Self::Standard,
            Self::Vintage,
        ].into_iter()
    }
}
