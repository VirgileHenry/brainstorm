#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Emblem,
    Enchantment,
    Hero,
    Instant,
    Kindred,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Vanguard,
}

impl std::str::FromStr for CardType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artifact" => Ok(Self::Artifact),
            "battle" => Ok(Self::Battle),
            "conspiracy" => Ok(Self::Conspiracy),
            "creature" => Ok(Self::Creature),
            "dungeon" => Ok(Self::Dungeon),
            "emblem" => Ok(Self::Emblem),
            "enchantment" => Ok(Self::Enchantment),
            "hero" => Ok(Self::Hero),
            "instant" => Ok(Self::Instant),
            "kindred" => Ok(Self::Kindred),
            "land" => Ok(Self::Land),
            "phenomenon" => Ok(Self::Phenomenon),
            "plane" => Ok(Self::Plane),
            "planeswalker" => Ok(Self::Planeswalker),
            "scheme" => Ok(Self::Scheme),
            "sorcery" => Ok(Self::Sorcery),
            "vanguard" => Ok(Self::Vanguard),
            other => Err(format!("Unknown CardType: {}", other.to_string())),
        }
    }
}

impl CardType {
    pub const VARIANT_COUNT: usize = 17;

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Artifact => "artifact",
            Self::Battle => "battle",
            Self::Conspiracy => "conspiracy",
            Self::Creature => "creature",
            Self::Dungeon => "dungeon",
            Self::Emblem => "emblem",
            Self::Enchantment => "enchantment",
            Self::Hero => "hero",
            Self::Instant => "instant",
            Self::Kindred => "kindred",
            Self::Land => "land",
            Self::Phenomenon => "phenomenon",
            Self::Plane => "plane",
            Self::Planeswalker => "planeswalker",
            Self::Scheme => "scheme",
            Self::Sorcery => "sorcery",
            Self::Vanguard => "vanguard",
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Artifact => 0,
            Self::Battle => 1,
            Self::Conspiracy => 2,
            Self::Creature => 3,
            Self::Dungeon => 4,
            Self::Emblem => 5,
            Self::Enchantment => 6,
            Self::Hero => 7,
            Self::Instant => 8,
            Self::Kindred => 9,
            Self::Land => 10,
            Self::Phenomenon => 11,
            Self::Plane => 12,
            Self::Planeswalker => 13,
            Self::Scheme => 14,
            Self::Sorcery => 15,
            Self::Vanguard => 16,
        }
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CardType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Artifact,
            Self::Battle,
            Self::Conspiracy,
            Self::Creature,
            Self::Dungeon,
            Self::Emblem,
            Self::Enchantment,
            Self::Hero,
            Self::Instant,
            Self::Kindred,
            Self::Land,
            Self::Phenomenon,
            Self::Plane,
            Self::Planeswalker,
            Self::Scheme,
            Self::Sorcery,
            Self::Vanguard,
        ].into_iter()
    }
}
