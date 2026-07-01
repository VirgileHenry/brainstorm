#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ArtifactType {
    Attraction,
    Blood,
    Bobblehead,
    Book,
    Clue,
    Contraption,
    Equipment,
    Food,
    Fortification,
    Gold,
    Incubator,
    Infinity,
    Junk,
    Lander,
    Map,
    Mutagen,
    Powerstone,
    Spacecraft,
    Stone,
    Treasure,
    Vehicle,
    Vibranium,
}

impl std::str::FromStr for ArtifactType {
    type Err = crate::ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attraction" => Ok(Self::Attraction),
            "blood" => Ok(Self::Blood),
            "bobblehead" => Ok(Self::Bobblehead),
            "book" => Ok(Self::Book),
            "clue" => Ok(Self::Clue),
            "contraption" => Ok(Self::Contraption),
            "equipment" => Ok(Self::Equipment),
            "food" => Ok(Self::Food),
            "fortification" => Ok(Self::Fortification),
            "gold" => Ok(Self::Gold),
            "incubator" => Ok(Self::Incubator),
            "infinity" => Ok(Self::Infinity),
            "junk" => Ok(Self::Junk),
            "lander" => Ok(Self::Lander),
            "map" => Ok(Self::Map),
            "mutagen" => Ok(Self::Mutagen),
            "powerstone" => Ok(Self::Powerstone),
            "spacecraft" => Ok(Self::Spacecraft),
            "stone" => Ok(Self::Stone),
            "treasure" => Ok(Self::Treasure),
            "vehicle" => Ok(Self::Vehicle),
            "vibranium" => Ok(Self::Vibranium),
            _ => Err(crate::ParsingError {
                item: "ArtifactType",
                message: "provided source does not match",
            }),
        }
    }
}

impl ArtifactType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Attraction => "attraction",
            Self::Blood => "blood",
            Self::Bobblehead => "bobblehead",
            Self::Book => "book",
            Self::Clue => "clue",
            Self::Contraption => "contraption",
            Self::Equipment => "equipment",
            Self::Food => "food",
            Self::Fortification => "fortification",
            Self::Gold => "gold",
            Self::Incubator => "incubator",
            Self::Infinity => "infinity",
            Self::Junk => "junk",
            Self::Lander => "lander",
            Self::Map => "map",
            Self::Mutagen => "mutagen",
            Self::Powerstone => "powerstone",
            Self::Spacecraft => "spacecraft",
            Self::Stone => "stone",
            Self::Treasure => "treasure",
            Self::Vehicle => "vehicle",
            Self::Vibranium => "vibranium",
        }
    }
}

impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ArtifactType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Attraction,
            Self::Blood,
            Self::Bobblehead,
            Self::Book,
            Self::Clue,
            Self::Contraption,
            Self::Equipment,
            Self::Food,
            Self::Fortification,
            Self::Gold,
            Self::Incubator,
            Self::Infinity,
            Self::Junk,
            Self::Lander,
            Self::Map,
            Self::Mutagen,
            Self::Powerstone,
            Self::Spacecraft,
            Self::Stone,
            Self::Treasure,
            Self::Vehicle,
            Self::Vibranium,
        ].into_iter()
    }
}
