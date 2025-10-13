pub struct CardType {
    pub supertypes: arrayvec::ArrayVec<mtg_data::Supertype, 4>,
    pub types: arrayvec::ArrayVec<TypeAndSubtypes, 4>,
}

impl CardType {
    pub fn parse(
        _type_line: &str,
        _raw_card: &mtg_cardbase::Card,
    ) -> Result<Self, String /* Fixme */> {
        Err(format!("Unimplemented"))
    }
}

pub enum TypeAndSubtypes {
    Artifact {
        subtypes: arrayvec::ArrayVec<mtg_data::ArtifactType, 4>,
    },
    Battle {
        subtypes: arrayvec::ArrayVec<mtg_data::BattleType, 4>,
    },
    Conspiracy,
    Creature {
        subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 4>,
    },
    Dungeon,
    Emblem,
    Enchantment {
        subtypes: arrayvec::ArrayVec<mtg_data::EnchantmentType, 4>,
    },
    Hero,
    Instant {
        subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
    },
    Kindred,
    Land {
        subtypes: arrayvec::ArrayVec<mtg_data::LandType, 4>,
    },
    Phenomenon,
    Plane,
    Planeswalker {
        subtypes: arrayvec::ArrayVec<mtg_data::PlaneswalkerType, 4>,
    },
    Scheme,
    Sorcery {
        subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
    },
    Vanguard,
}
