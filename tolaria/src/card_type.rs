use std::str::FromStr;

pub struct CardType {
    pub supertypes: arrayvec::ArrayVec<mtg_data::Supertype, 4>,
    pub types: arrayvec::ArrayVec<TypeAndSubtypes, 4>,
}

impl CardType {
    pub fn parse(
        type_line: &str,
        _raw_card: &mtg_cardbase::Card,
    ) -> Result<Self, String /* Fixme */> {
        use std::str::FromStr;

        // Fixme: perhaps prevent dusplicates ? for now we will happily parse a "Legendary Legendary Creature"

        let tokens_regex = regex::Regex::new(r"\b\w+\b")
            .map_err(|e| format!("Failed to compile the tokens regex: {e}"))?;

        let mut tokens = tokens_regex
            .find_iter(type_line)
            .map(|m| m.as_str())
            .peekable();

        /* Parse supertypes first */
        let mut supertypes = arrayvec::ArrayVec::new();
        while let Some(token) = tokens.peek() {
            match mtg_data::Supertype::from_str(token) {
                Ok(supertype) => {
                    supertypes.push(supertype);
                    let _ = tokens.next(); /* Token was peeked, pop it out */
                }
                Err(_) => break,
            }
        }

        /* Parse all types then */
        let mut types = arrayvec::ArrayVec::new();
        while let Some(token) = tokens.peek() {
            match mtg_data::CardType::from_str(token) {
                Ok(card_type) => {
                    types.push(TypeAndSubtypes::empty_from_card_type(card_type));
                    let _ = tokens.next(); /* Token was peeked, pop it out */
                }
                Err(_) => break,
            }
        }

        /* Finally, parse subtypes and fill them into the corresponding types */
        while let Some(token) = tokens.next() {
            for card_type in types.iter_mut() {
                if card_type.try_parse_and_insert(token).is_ok() {
                    continue; /* Subtype belongs to a known type, go to next */
                }
            }
            /* This is the error path, we can allocate */
            let types_display = types
                .iter()
                .map(|t| t.card_type().as_str())
                .collect::<Vec<_>>();
            return Err(format!(
                "Subtype {token} does not fit any of the parsed types: {}",
                types_display.join(", ")
            ));
        }

        Ok(CardType { supertypes, types })
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

impl TypeAndSubtypes {
    fn empty_from_card_type(value: mtg_data::CardType) -> Self {
        match value {
            mtg_data::CardType::Artifact => Self::Artifact {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Battle => Self::Battle {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Conspiracy => Self::Conspiracy,
            mtg_data::CardType::Creature => Self::Creature {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Dungeon => Self::Dungeon,
            mtg_data::CardType::Emblem => Self::Emblem,
            mtg_data::CardType::Enchantment => Self::Enchantment {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Hero => Self::Hero,
            mtg_data::CardType::Instant => Self::Instant {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Kindred => Self::Kindred,
            mtg_data::CardType::Land => Self::Land {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Phenomenon => Self::Phenomenon,
            mtg_data::CardType::Plane => Self::Plane,
            mtg_data::CardType::Planeswalker => Self::Planeswalker {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Scheme => Self::Scheme,
            mtg_data::CardType::Sorcery => Self::Sorcery {
                subtypes: arrayvec::ArrayVec::new(),
            },
            mtg_data::CardType::Vanguard => Self::Vanguard,
        }
    }

    fn card_type(&self) -> mtg_data::CardType {
        match self {
            TypeAndSubtypes::Artifact { .. } => mtg_data::CardType::Artifact,
            TypeAndSubtypes::Battle { .. } => mtg_data::CardType::Battle,
            TypeAndSubtypes::Conspiracy => mtg_data::CardType::Conspiracy,
            TypeAndSubtypes::Creature { .. } => mtg_data::CardType::Creature,
            TypeAndSubtypes::Dungeon => mtg_data::CardType::Dungeon,
            TypeAndSubtypes::Emblem => mtg_data::CardType::Emblem,
            TypeAndSubtypes::Enchantment { .. } => mtg_data::CardType::Enchantment,
            TypeAndSubtypes::Hero => mtg_data::CardType::Hero,
            TypeAndSubtypes::Instant { .. } => mtg_data::CardType::Instant,
            TypeAndSubtypes::Kindred => mtg_data::CardType::Kindred,
            TypeAndSubtypes::Land { .. } => mtg_data::CardType::Land,
            TypeAndSubtypes::Phenomenon => mtg_data::CardType::Phenomenon,
            TypeAndSubtypes::Plane => mtg_data::CardType::Plane,
            TypeAndSubtypes::Planeswalker { .. } => mtg_data::CardType::Planeswalker,
            TypeAndSubtypes::Scheme => mtg_data::CardType::Scheme,
            TypeAndSubtypes::Sorcery { .. } => mtg_data::CardType::Sorcery,
            TypeAndSubtypes::Vanguard => mtg_data::CardType::Vanguard,
        }
    }

    fn try_parse_and_insert(&mut self, token: &str) -> Result<(), String /* Fixme */> {
        match self {
            TypeAndSubtypes::Artifact { subtypes } => {
                let artifact_type = mtg_data::ArtifactType::from_str(token)?;
                subtypes.push(artifact_type);
                Ok(())
            }
            TypeAndSubtypes::Battle { subtypes } => {
                let battle_type = mtg_data::BattleType::from_str(token)?;
                subtypes.push(battle_type);
                Ok(())
            }
            TypeAndSubtypes::Conspiracy => Err(format!("No subtypes for Conspiracy")),
            TypeAndSubtypes::Creature { subtypes } => {
                let creature_type = mtg_data::CreatureType::from_str(token)?;
                subtypes.push(creature_type);
                Ok(())
            }
            TypeAndSubtypes::Dungeon => Err(format!("No subtypes for Dungeon")),
            TypeAndSubtypes::Emblem => Err(format!("No subtypes for Emblem")),
            TypeAndSubtypes::Enchantment { subtypes } => {
                let enchantment_type = mtg_data::EnchantmentType::from_str(token)?;
                subtypes.push(enchantment_type);
                Ok(())
            }
            TypeAndSubtypes::Hero => Err(format!("No subtypes for Hero")),
            TypeAndSubtypes::Instant { subtypes } => {
                let spell_type = mtg_data::SpellType::from_str(token)?;
                subtypes.push(spell_type);
                Ok(())
            }
            TypeAndSubtypes::Kindred => Err(format!("No subtypes for Kindred")),
            TypeAndSubtypes::Land { subtypes } => {
                let land_type = mtg_data::LandType::from_str(token)?;
                subtypes.push(land_type);
                Ok(())
            }
            TypeAndSubtypes::Phenomenon => Err(format!("No subtypes for Phenomenon")),
            TypeAndSubtypes::Plane => Err(format!("No subtypes for Plane")),
            TypeAndSubtypes::Planeswalker { subtypes } => {
                let planeswalker_type = mtg_data::PlaneswalkerType::from_str(token)?;
                subtypes.push(planeswalker_type);
                Ok(())
            }
            TypeAndSubtypes::Scheme => Err(format!("No subtypes for Scheme")),
            TypeAndSubtypes::Sorcery { subtypes } => {
                let spell_type = mtg_data::SpellType::from_str(token)?;
                subtypes.push(spell_type);
                Ok(())
            }
            TypeAndSubtypes::Vanguard => Err(format!("No subtypes for Vanguard")),
        }
    }
}
