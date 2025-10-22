#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LandType {
    Cave,
    Cloud,
    Desert,
    Forest,
    Gate,
    Island,
    Lair,
    Locus,
    Mine,
    Mountain,
    Plains,
    Planet,
    PowerPlant,
    Sphere,
    Swamp,
    Tower,
    Town,
    Urzas,
}

impl std::str::FromStr for LandType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cave" => Ok(Self::Cave),
            "cloud" => Ok(Self::Cloud),
            "desert" => Ok(Self::Desert),
            "forest" => Ok(Self::Forest),
            "gate" => Ok(Self::Gate),
            "island" => Ok(Self::Island),
            "lair" => Ok(Self::Lair),
            "locus" => Ok(Self::Locus),
            "mine" => Ok(Self::Mine),
            "mountain" => Ok(Self::Mountain),
            "plains" => Ok(Self::Plains),
            "planet" => Ok(Self::Planet),
            "power-plant" => Ok(Self::PowerPlant),
            "sphere" => Ok(Self::Sphere),
            "swamp" => Ok(Self::Swamp),
            "tower" => Ok(Self::Tower),
            "town" => Ok(Self::Town),
            "urza's" => Ok(Self::Urzas),
            other => Err(format!("Unknown LandType: {}", other.to_string())),
        }
    }
}

impl LandType {
    pub const VARIANT_COUNT: usize = 18;

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cave => "cave",
            Self::Cloud => "cloud",
            Self::Desert => "desert",
            Self::Forest => "forest",
            Self::Gate => "gate",
            Self::Island => "island",
            Self::Lair => "lair",
            Self::Locus => "locus",
            Self::Mine => "mine",
            Self::Mountain => "mountain",
            Self::Plains => "plains",
            Self::Planet => "planet",
            Self::PowerPlant => "power-plant",
            Self::Sphere => "sphere",
            Self::Swamp => "swamp",
            Self::Tower => "tower",
            Self::Town => "town",
            Self::Urzas => "urza's",
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Cave => 0,
            Self::Cloud => 1,
            Self::Desert => 2,
            Self::Forest => 3,
            Self::Gate => 4,
            Self::Island => 5,
            Self::Lair => 6,
            Self::Locus => 7,
            Self::Mine => 8,
            Self::Mountain => 9,
            Self::Plains => 10,
            Self::Planet => 11,
            Self::PowerPlant => 12,
            Self::Sphere => 13,
            Self::Swamp => 14,
            Self::Tower => 15,
            Self::Town => 16,
            Self::Urzas => 17,
        }
    }
}

impl std::fmt::Display for LandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl LandType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Cave,
            Self::Cloud,
            Self::Desert,
            Self::Forest,
            Self::Gate,
            Self::Island,
            Self::Lair,
            Self::Locus,
            Self::Mine,
            Self::Mountain,
            Self::Plains,
            Self::Planet,
            Self::PowerPlant,
            Self::Sphere,
            Self::Swamp,
            Self::Tower,
            Self::Town,
            Self::Urzas,
        ].into_iter()
    }
}
