#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CardType {
    supertypes: arrayvec::ArrayVec<mtg_data::Supertype, 4>,
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
        raw_card: &mtg_cardbase::Card,
    ) -> Result<Self, String /* Fixme */> {
        use std::str::FromStr;

        let mut result = Self::empty();

        lazy_static::lazy_static!(
            static ref tokens_regex: regex::Regex = regex::Regex::new(r"\b\w+\b")
                .expect("Failed to compile the tokens regex");
        );

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
                Ok(card_type) => {
                    match card_type {
                        mtg_data::CardType::Artifact => match result.artifact {
                            None => {
                                result.artifact = Some(ArtifactSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Artifact type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Battle => match result.battle {
                            None => {
                                result.battle = Some(BattleSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Battle type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Conspiracy => match result.conspiracy {
                            None => result.conspiracy = Some(ConspiracySubtype),
                            Some(_) => {
                                return Err(format!(
                                    "Consiparacy type present twice in card type!"
                                ));
                            }
                        },
                        mtg_data::CardType::Creature => match result.creature {
                            None => {
                                result.creature = Some(CreatureSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Creature type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Dungeon => match result.dungeon {
                            None => result.dungeon = Some(DungeonSubtype),
                            Some(_) => {
                                return Err(format!("Dungeon type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Emblem => match result.emblem {
                            None => result.emblem = Some(EmblemSubtype),
                            Some(_) => {
                                return Err(format!("Emblem type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Enchantment => match result.enchantment {
                            None => {
                                result.enchantment = Some(EnchantmentSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!(
                                    "Enchantment type present twice in card type!"
                                ));
                            }
                        },
                        mtg_data::CardType::Hero => match result.hero {
                            None => result.hero = Some(HeroSubtype),
                            Some(_) => {
                                return Err(format!("Hero type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Instant => match result.instant {
                            None => {
                                result.instant = Some(InstantSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Instant type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Kindred => match result.kindred {
                            None => {
                                result.kindred = Some(KindredSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Kindred type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Land => match result.land {
                            None => {
                                result.land = Some(LandSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Land type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Phenomenon => match result.phenomenon {
                            None => result.phenomenon = Some(PhenomenonSubtype),
                            Some(_) => {
                                return Err(format!("Phenomenon type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Plane => match result.plane {
                            None => result.plane = Some(PlaneSubtype),
                            Some(_) => {
                                return Err(format!("Plane type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Planeswalker => match result.planeswalker {
                            None => {
                                let loyalty = raw_card.loyalty.ok_or_else(|| {
                                    format!("No loyalty field on card with planeswalker type!")
                                })?;
                                result.planeswalker = Some(PlaneswalkerSubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                    loyalty: loyalty.parse().map_err(|e| {
                                        format!("Failed to parse loyalty \"{loyalty}\": {e}")
                                    })?,
                                })
                            }
                            Some(_) => {
                                return Err(format!(
                                    "Planeswalker type present twice in card type!"
                                ));
                            }
                        },
                        mtg_data::CardType::Scheme => match result.scheme {
                            None => result.scheme = Some(SchemeSubtype),
                            Some(_) => {
                                return Err(format!("Scheme type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Sorcery => match result.sorcery {
                            None => {
                                result.sorcery = Some(SorcerySubtype {
                                    subtypes: arrayvec::ArrayVec::new(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Sorcery type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Vanguard => match result.vanguard {
                            None => result.vanguard = Some(VanguardSubtype),
                            Some(_) => {
                                return Err(format!("Vanguard type present twice in card type!"));
                            }
                        },
                    }
                    let _ = tokens.next();
                }
                Err(_) => break,
            }
        }

        while let Some(token) = tokens.next() {
            if let Some(subtype) = &mut result.artifact {
                if let Ok(new_subtype) = mtg_data::ArtifactType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.battle {
                if let Ok(new_subtype) = mtg_data::BattleType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.creature {
                if let Ok(new_subtype) = mtg_data::CreatureType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.enchantment {
                if let Ok(new_subtype) = mtg_data::EnchantmentType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.instant {
                if let Ok(new_subtype) = mtg_data::SpellType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.kindred {
                if let Ok(new_subtype) = mtg_data::CreatureType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.land {
                if let Ok(new_subtype) = mtg_data::LandType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.planeswalker {
                if let Ok(new_subtype) = mtg_data::PlaneswalkerType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.sorcery {
                if let Ok(new_subtype) = mtg_data::SpellType::from_str(token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return Err(format!("Subtype {new_subtype} present twice in card type!"));
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            /* If we arrive here, no type managed to validated the given subtype, that's an error! */
            return Err(format!("subtype {token} does not fit any card types!"));
        }

        Ok(result)
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for supertype in self.supertypes.iter() {
            write!(f, "{supertype} ")?;
        }

        if self.artifact.is_some() {
            write!(f, "{} ", mtg_data::CardType::Artifact)?;
        }
        if self.battle.is_some() {
            write!(f, "{} ", mtg_data::CardType::Battle)?;
        }
        if self.conspiracy.is_some() {
            write!(f, "{} ", mtg_data::CardType::Conspiracy)?;
        }
        if self.creature.is_some() {
            write!(f, "{} ", mtg_data::CardType::Creature)?;
        }
        if self.dungeon.is_some() {
            write!(f, "{} ", mtg_data::CardType::Dungeon)?;
        }
        if self.emblem.is_some() {
            write!(f, "{} ", mtg_data::CardType::Emblem)?;
        }
        if self.enchantment.is_some() {
            write!(f, "{} ", mtg_data::CardType::Enchantment)?;
        }
        if self.hero.is_some() {
            write!(f, "{} ", mtg_data::CardType::Hero)?;
        }
        if self.instant.is_some() {
            write!(f, "{} ", mtg_data::CardType::Instant)?;
        }
        if self.kindred.is_some() {
            write!(f, "{} ", mtg_data::CardType::Kindred)?;
        }
        if self.land.is_some() {
            write!(f, "{} ", mtg_data::CardType::Land)?;
        }
        if self.phenomenon.is_some() {
            write!(f, "{} ", mtg_data::CardType::Phenomenon)?;
        }
        if self.plane.is_some() {
            write!(f, "{} ", mtg_data::CardType::Plane)?;
        }
        if self.planeswalker.is_some() {
            write!(f, "{} ", mtg_data::CardType::Planeswalker)?;
        }
        if self.scheme.is_some() {
            write!(f, "{} ", mtg_data::CardType::Scheme)?;
        }
        if self.sorcery.is_some() {
            write!(f, "{} ", mtg_data::CardType::Sorcery)?;
        }
        if self.vanguard.is_some() {
            write!(f, "{} ", mtg_data::CardType::Vanguard)?;
        }

        write!(f, "— ")?;

        if let Some(subtype) = &self.artifact {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.battle {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.creature {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.enchantment {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.instant {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.kindred {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.land {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.planeswalker {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }
        if let Some(subtype) = &self.sorcery {
            for subtype in subtype.subtypes.iter() {
                write!(f, "{subtype} ")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtifactSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::ArtifactType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::BattleType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConspiracySubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreatureSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DungeonSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmblemSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnchantmentSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::EnchantmentType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeroSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstantSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KindredSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LandSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::LandType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhenomenonSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaneSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaneswalkerSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::PlaneswalkerType, 4>,
    loyalty: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SchemeSubtype;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SorcerySubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VanguardSubtype;
