#[derive(Debug, Clone)]
pub struct CardType {
    pub supertypes: arrayvec::ArrayVec<mtg_data::Supertype, 4>,
    artifact: Option<ArtifactSubtype>,
    battle: Option<BattleSubtype>,
    conspiracy: Option<ConspiracySubtype>,
    creature: Option<CreatureSubtype>,
    dungeon: Option<DungeonSubtype>,
    emblem: Option<EmblemSubtype>,
    enchantment: Option<EnchantmentSubtype>,
    hero: Option<HeroSubtype>,
    instant: Option<InstantSubtype>,
    kindred: Option<KindredSubtype>,
    land: Option<LandSubtype>,
    phenomenon: Option<PhenomenonSubtype>,
    plane: Option<PlaneSubtype>,
    planeswalker: Option<PlaneswalkerSubtype>,
    scheme: Option<SchemeSubtype>,
    sorcery: Option<SorcerySubtype>,
    vanguard: Option<VanguardSubtype>,
}

impl CardType {
    fn empty() -> CardType {
        CardType {
            supertypes: arrayvec::ArrayVec::new(),
            artifact: None,
            battle: None,
            conspiracy: None,
            creature: None,
            dungeon: None,
            emblem: None,
            enchantment: None,
            hero: None,
            instant: None,
            kindred: None,
            land: None,
            phenomenon: None,
            plane: None,
            planeswalker: None,
            scheme: None,
            sorcery: None,
            vanguard: None,
        }
    }
    pub fn parse(
        type_line: &str,
        _raw_card: &mtg_cardbase::Card,
    ) -> Result<Self, String /* Fixme */> {
        use std::str::FromStr;

        let mut result = Self::empty();

        let tokens_regex = regex::Regex::new(r"\b\w+\b")
            .map_err(|e| format!("Failed to compile the tokens regex: {e}"))?;

        let mut tokens = tokens_regex
            .find_iter(type_line)
            .map(|m| m.as_str())
            .peekable();

        /* Parse supertypes first */
        while let Some(token) = tokens.peek() {
            match mtg_data::Supertype::from_str(token) {
                Ok(supertype) => {
                    if result.supertypes.contains(&supertype) {
                        return Err(format!("Duplicate super type {supertype}"));
                    } else {
                        result.supertypes.push(supertype);
                        let _ = tokens.next(); /* Token was peeked, pop it out */
                    }
                }
                Err(_) => break,
            }
        }

        /* Parse all types then */
        while let Some(token) = tokens.peek() {
            match mtg_data::CardType::from_str(token) {
                Ok(card_type) => match card_type {
                    mtg_data::CardType::Artifact => {
                        result.artifact = Some(ArtifactSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Battle => {
                        result.battle = Some(BattleSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Conspiracy => result.conspiracy = Some(ConspiracySubtype),
                    mtg_data::CardType::Creature => {
                        result.creature = Some(CreatureSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Dungeon => result.dungeon = Some(DungeonSubtype),
                    mtg_data::CardType::Emblem => result.emblem = Some(EmblemSubtype),
                    mtg_data::CardType::Enchantment => {
                        result.enchantment = Some(EnchantmentSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Hero => result.hero = Some(HeroSubtype),
                    mtg_data::CardType::Instant => {
                        result.instant = Some(InstantSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Kindred => result.kindred = Some(KindredSubtype),
                    mtg_data::CardType::Land => {
                        result.land = Some(LandSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Phenomenon => result.phenomenon = Some(PhenomenonSubtype),
                    mtg_data::CardType::Plane => result.plane = Some(PlaneSubtype),
                    mtg_data::CardType::Planeswalker => {
                        result.planeswalker = Some(PlaneswalkerSubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Scheme => result.scheme = Some(SchemeSubtype),
                    mtg_data::CardType::Sorcery => {
                        result.sorcery = Some(SorcerySubtype {
                            subtypes: arrayvec::ArrayVec::new(),
                        })
                    }
                    mtg_data::CardType::Vanguard => result.vanguard = Some(VanguardSubtype),
                },
                Err(_) => break,
            }
        }

        // Todo: parse subtypes also

        Ok(result)
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Todo")
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::ArtifactType, 4>,
}

#[derive(Debug, Clone)]
pub struct BattleSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::BattleType, 4>,
}

#[derive(Debug, Clone)]
pub struct ConspiracySubtype;

#[derive(Debug, Clone)]
pub struct CreatureSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 4>,
}

#[derive(Debug, Clone)]
pub struct DungeonSubtype;

#[derive(Debug, Clone)]
pub struct EmblemSubtype;

#[derive(Debug, Clone)]
pub struct EnchantmentSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::EnchantmentType, 4>,
}

#[derive(Debug, Clone)]
pub struct HeroSubtype;

#[derive(Debug, Clone)]
pub struct InstantSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

#[derive(Debug, Clone)]
pub struct KindredSubtype;

#[derive(Debug, Clone)]
pub struct LandSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::LandType, 4>,
}

#[derive(Debug, Clone)]
pub struct PhenomenonSubtype;

#[derive(Debug, Clone)]
pub struct PlaneSubtype;

#[derive(Debug, Clone)]
pub struct PlaneswalkerSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::PlaneswalkerType, 4>,
}

#[derive(Debug, Clone)]
pub struct SchemeSubtype;

#[derive(Debug, Clone)]
pub struct SorcerySubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

#[derive(Debug, Clone)]
pub struct VanguardSubtype;
