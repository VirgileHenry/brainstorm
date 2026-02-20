use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

const MAX_SUPERTYPES_COUNT: usize = 4;

/// The type line of a mtg card contains all the card types.
///
/// From the comprehensive rules:
/// Part of a card. The type line is printed directly below the illustration
/// and contains the card’s card type(s), subtype(s), and/or supertype(s).
/// See rule 205, “Type Line.”
///
/// See also: https://mtg.fandom.com/wiki/Type_line
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TypeLine {
    supertypes: arrayvec::ArrayVec<mtg_data::Supertype, MAX_SUPERTYPES_COUNT>,
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

impl TypeLine {
    fn empty() -> TypeLine {
        TypeLine {
            supertypes: arrayvec::ArrayVec::new_const(),
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

    pub fn parse(type_line: &str, raw_card: &mtg_cardbase::Card) -> Result<Self, String /* Fixme */> {
        use std::str::FromStr;

        let mut result = Self::empty();

        lazy_static::lazy_static!(
            static ref tokens_regex: regex::Regex = regex::Regex::new(r"\b\w+\b")
                .expect("Failed to compile the tokens regex");
        );

        let mut tokens = tokens_regex.find_iter(type_line).map(|m| m.as_str()).peekable();

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
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Artifact type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Battle => match result.battle {
                            None => {
                                result.battle = Some(BattleSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Battle type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Conspiracy => match result.conspiracy {
                            None => result.conspiracy = Some(ConspiracySubtype),
                            Some(_) => {
                                return Err(format!("Consiparacy type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Creature => match result.creature {
                            None => {
                                result.creature = Some(CreatureSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
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
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Enchantment type present twice in card type!"));
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
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Instant type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Kindred => match result.kindred {
                            None => {
                                result.kindred = Some(KindredSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                })
                            }
                            Some(_) => {
                                return Err(format!("Kindred type present twice in card type!"));
                            }
                        },
                        mtg_data::CardType::Land => match result.land {
                            None => {
                                result.land = Some(LandSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
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
                                let loyalty = raw_card
                                    .loyalty
                                    .as_ref()
                                    .ok_or_else(|| format!("No loyalty field on card with planeswalker type!"))?;
                                result.planeswalker = Some(PlaneswalkerSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    loyalty: loyalty
                                        .parse()
                                        .map_err(|e| format!("Failed to parse loyalty \"{loyalty}\": {e}"))?,
                                })
                            }
                            Some(_) => {
                                return Err(format!("Planeswalker type present twice in card type!"));
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
                                    subtypes: arrayvec::ArrayVec::new_const(),
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

    pub fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        let mut result = arrayvec::ArrayVec::new_const();

        if self.artifact.is_some() {
            result.push(mtg_data::CardType::Artifact)
        }
        if self.battle.is_some() {
            result.push(mtg_data::CardType::Battle)
        }
        if self.conspiracy.is_some() {
            result.push(mtg_data::CardType::Conspiracy)
        }
        if self.creature.is_some() {
            result.push(mtg_data::CardType::Creature)
        }
        if self.dungeon.is_some() {
            result.push(mtg_data::CardType::Dungeon)
        }
        if self.emblem.is_some() {
            result.push(mtg_data::CardType::Emblem)
        }
        if self.enchantment.is_some() {
            result.push(mtg_data::CardType::Enchantment)
        }
        if self.hero.is_some() {
            result.push(mtg_data::CardType::Hero)
        }
        if self.instant.is_some() {
            result.push(mtg_data::CardType::Instant)
        }
        if self.kindred.is_some() {
            result.push(mtg_data::CardType::Kindred)
        }
        if self.land.is_some() {
            result.push(mtg_data::CardType::Land)
        }
        if self.phenomenon.is_some() {
            result.push(mtg_data::CardType::Phenomenon)
        }
        if self.plane.is_some() {
            result.push(mtg_data::CardType::Plane)
        }
        if self.planeswalker.is_some() {
            result.push(mtg_data::CardType::Planeswalker)
        }
        if self.scheme.is_some() {
            result.push(mtg_data::CardType::Scheme)
        }
        if self.sorcery.is_some() {
            result.push(mtg_data::CardType::Sorcery)
        }
        if self.vanguard.is_some() {
            result.push(mtg_data::CardType::Vanguard)
        }

        result
    }
}

impl AbilityTreeNode for TypeLine {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TypeLineIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();

        /* Create an iterator of dummy empty nodes as ability tree */
        let dummy_nodes = std::iter::repeat(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::empty_node());
        let dummy_nodes = dummy_nodes.map(|c| c as &dyn AbilityTreeNode);

        /* Iterate over the specifiers, filled up with dummy nodes if there are less than OR_OF_AND_LIST_INNER_ARRAY_LENGTH specifiers */
        for supertype in self
            .supertypes
            .iter()
            .map(|s| s as &dyn AbilityTreeNode)
            .chain(dummy_nodes)
            .take(MAX_SUPERTYPES_COUNT)
        {
            children.push(supertype as &dyn AbilityTreeNode);
        }

        let none_node = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();

        match self.artifact.as_ref() {
            Some(artifact) => children.push(artifact as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.battle.as_ref() {
            Some(battle) => children.push(battle as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.conspiracy.as_ref() {
            Some(conspiracy) => children.push(conspiracy as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.creature.as_ref() {
            Some(creature) => children.push(creature as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.dungeon.as_ref() {
            Some(dungeon) => children.push(dungeon as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.emblem.as_ref() {
            Some(emblem) => children.push(emblem as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.enchantment.as_ref() {
            Some(enchantment) => children.push(enchantment as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.hero.as_ref() {
            Some(hero) => children.push(hero as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.instant.as_ref() {
            Some(instant) => children.push(instant as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.kindred.as_ref() {
            Some(kindred) => children.push(kindred as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.land.as_ref() {
            Some(land) => children.push(land as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.phenomenon.as_ref() {
            Some(phenomenon) => children.push(phenomenon as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.plane.as_ref() {
            Some(plane) => children.push(plane as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.planeswalker.as_ref() {
            Some(planeswalker) => children.push(planeswalker as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.scheme.as_ref() {
            Some(scheme) => children.push(scheme as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.sorcery.as_ref() {
            Some(sorcery) => children.push(sorcery as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }
        match self.vanguard.as_ref() {
            Some(vanguard) => children.push(vanguard as &dyn AbilityTreeNode),
            None => children.push(none_node as &dyn AbilityTreeNode),
        }

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "type line:")?;
        out.push_final_branch()?;
        write!(out, "{self}")?;
        out.pop_branch();
        Ok(())
    }
}

impl std::fmt::Display for TypeLine {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ArtifactSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::ArtifactType, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for ArtifactSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::ArtifactSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "artifact type")?;
        out.push_final_branch()?;
        write!(out, "artifact subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct BattleSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::BattleType, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for BattleSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::BattleSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "battle type")?;
        out.push_final_branch()?;
        write!(out, "battle subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ConspiracySubtype;

impl AbilityTreeNode for ConspiracySubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::ConspiracySubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "conspiracy type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CreatureSubtype {
    /* Fixme: power / toughness ? */
    subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for CreatureSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::CreatureSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        /* Fixme: power / toughness, and reduce the number of creature subtypes */
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature type")?;
        out.push_final_branch()?;
        write!(out, "creature subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct DungeonSubtype;

impl AbilityTreeNode for DungeonSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::DungeonSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "dungeon type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct EmblemSubtype;

impl AbilityTreeNode for EmblemSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::EmblemSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "emblem type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct EnchantmentSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::EnchantmentType, 4>,
}

impl AbilityTreeNode for EnchantmentSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::EnchantmentSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enchantment type")?;
        out.push_final_branch()?;
        write!(out, "enchantment subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct HeroSubtype;

impl AbilityTreeNode for HeroSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::HeroSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "emblem type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct InstantSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

impl AbilityTreeNode for InstantSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::InstantSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "instant type")?;
        out.push_final_branch()?;
        write!(out, "instant subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct KindredSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::CreatureType, 4>,
}

impl AbilityTreeNode for KindredSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::KindredSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "kindred type")?;
        out.push_final_branch()?;
        write!(out, "kindred subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct LandSubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::LandType, 4>,
}

impl AbilityTreeNode for LandSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::LandSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "land type")?;
        out.push_final_branch()?;
        write!(out, "land subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PhenomenonSubtype;

impl AbilityTreeNode for PhenomenonSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::PhenomenonSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "phenomenon type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlaneSubtype;

impl AbilityTreeNode for PlaneSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::PlaneSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "plane type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlaneswalkerSubtype {
    loyalty: u64,
    subtypes: arrayvec::ArrayVec<mtg_data::PlaneswalkerType, 4>,
}

impl AbilityTreeNode for PlaneswalkerSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::PlaneswalkerSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: terrible for the AI */
        self.loyalty.to_le_bytes().into_iter().collect()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "planeswalker type")?;
        out.push_final_branch()?;
        write!(out, "planeswalker subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SchemeSubtype;

impl AbilityTreeNode for SchemeSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::SchemeSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "scheme type")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SorcerySubtype {
    subtypes: arrayvec::ArrayVec<mtg_data::SpellType, 4>,
}

impl AbilityTreeNode for SorcerySubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::SorcerySubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for subtype in self.subtypes.iter() {
            children.push(subtype as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "sorcery type")?;
        out.push_final_branch()?;
        write!(out, "sorcery subtypes:")?;
        for (i, subtype) in self.subtypes.iter().enumerate() {
            if i == self.subtypes.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            subtype.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct VanguardSubtype;

impl AbilityTreeNode for VanguardSubtype {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TypeLineNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::TypeLine(TypeLineNodeKind::VanguardSubtype).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "vanguard type")?;
        Ok(())
    }
}
