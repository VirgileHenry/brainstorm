use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::lexer::IntoToken;

const MAX_SUPERTYPES_COUNT: usize = 4;

/// The type line of a mtg card contains all the card types.
///
/// From the comprehensive rules:
/// Part of a card. The type line is printed directly below the illustration
/// and contains the card’s card type(s), subtype(s), and/or supertype(s).
/// See rule 205, “Type Line.”
///
/// See also: <https://mtg.fandom.com/wiki/Type_line>
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TypeLine {
    pub supertypes: arrayvec::ArrayVec<crate::ability_tree::object::Supertype, MAX_SUPERTYPES_COUNT>,
    pub artifact: Option<ArtifactSubtype>,
    pub battle: Option<BattleSubtype>,
    pub conspiracy: Option<ConspiracySubtype>,
    pub creature: Option<CreatureSubtype>,
    pub dungeon: Option<DungeonSubtype>,
    pub emblem: Option<EmblemSubtype>,
    pub enchantment: Option<EnchantmentSubtype>,
    pub hero: Option<HeroSubtype>,
    pub instant: Option<InstantSubtype>,
    pub kindred: Option<KindredSubtype>,
    pub land: Option<LandSubtype>,
    pub phenomenon: Option<PhenomenonSubtype>,
    pub plane: Option<PlaneSubtype>,
    pub planeswalker: Option<PlaneswalkerSubtype>,
    pub scheme: Option<SchemeSubtype>,
    pub sorcery: Option<SorcerySubtype>,
    pub vanguard: Option<VanguardSubtype>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn creature_token(
        creature_subtypes: &[crate::ability_tree::object::CreatureSubtype],
        #[cfg(feature = "spanned_tree")] token_span: crate::ability_tree::span::TreeSpan,
        #[cfg(feature = "spanned_tree")] subtype_span: crate::ability_tree::span::TreeSpan,
    ) -> Self {
        let mut result = Self::empty();
        result.supertypes.push(crate::ability_tree::object::Supertype {
            supertype: mtg_data::Supertype::Token,
            #[cfg(feature = "spanned_tree")]
            span: token_span,
        });
        result.creature = Some(CreatureSubtype {
            subtypes: {
                let mut subtypes = arrayvec::ArrayVec::new_const();
                for subtype in creature_subtypes.iter() {
                    subtypes.push(subtype.clone());
                }
                subtypes
            },
            #[cfg(feature = "spanned_tree")]
            span: token_span.merge(&subtype_span),
        });
        result
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

        for supertype in self.supertypes.iter() {
            supertype.display(out)?;
        }

        if self.artifact.is_some() {
            write!(out, "{} ", mtg_data::CardType::Artifact)?;
        }
        if self.battle.is_some() {
            write!(out, "{} ", mtg_data::CardType::Battle)?;
        }
        if self.conspiracy.is_some() {
            write!(out, "{} ", mtg_data::CardType::Conspiracy)?;
        }
        if self.creature.is_some() {
            write!(out, "{} ", mtg_data::CardType::Creature)?;
        }
        if self.dungeon.is_some() {
            write!(out, "{} ", mtg_data::CardType::Dungeon)?;
        }
        if self.emblem.is_some() {
            write!(out, "{} ", mtg_data::CardType::Emblem)?;
        }
        if self.enchantment.is_some() {
            write!(out, "{} ", mtg_data::CardType::Enchantment)?;
        }
        if self.hero.is_some() {
            write!(out, "{} ", mtg_data::CardType::Hero)?;
        }
        if self.instant.is_some() {
            write!(out, "{} ", mtg_data::CardType::Instant)?;
        }
        if self.kindred.is_some() {
            write!(out, "{} ", mtg_data::CardType::Kindred)?;
        }
        if self.land.is_some() {
            write!(out, "{} ", mtg_data::CardType::Land)?;
        }
        if self.phenomenon.is_some() {
            write!(out, "{} ", mtg_data::CardType::Phenomenon)?;
        }
        if self.plane.is_some() {
            write!(out, "{} ", mtg_data::CardType::Plane)?;
        }
        if self.planeswalker.is_some() {
            write!(out, "{} ", mtg_data::CardType::Planeswalker)?;
        }
        if self.scheme.is_some() {
            write!(out, "{} ", mtg_data::CardType::Scheme)?;
        }
        if self.sorcery.is_some() {
            write!(out, "{} ", mtg_data::CardType::Sorcery)?;
        }
        if self.vanguard.is_some() {
            write!(out, "{} ", mtg_data::CardType::Vanguard)?;
        }

        write!(out, "— ")?;

        if let Some(subtype) = &self.artifact {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.battle {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.creature {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.enchantment {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.instant {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.kindred {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.land {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.planeswalker {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }
        if let Some(subtype) = &self.sorcery {
            for subtype in subtype.subtypes.iter() {
                subtype.display(out)?;
            }
        }

        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "type line"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for TypeLine {
    fn dummy_init() -> Self {
        Self {
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
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl IntoToken for TypeLine {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        let mut result = Self::empty();

        lazy_static::lazy_static!(
            static ref tokens_regex: regex::Regex = regex::Regex::new(r"\b\w+\b")
                .expect("Failed to compile the tokens regex");
        );

        let mut spans = tokens_regex
            .find_iter(span.text)
            .map(|m| crate::lexer::Span {
                start: span.start + m.start(),
                length: m.len(),
                text: m.as_str(),
            })
            .peekable();

        /* Parse supertypes first */
        while let Some(token) = spans.peek() {
            match crate::ability_tree::object::Supertype::try_from_span(token) {
                Some(supertype) => {
                    if result.supertypes.contains(&supertype) {
                        return None;
                    } else {
                        result.supertypes.push(supertype);
                        let _ = spans.next(); /* Token was peeked, pop it out */
                    }
                }
                None => break,
            }
        }

        /* Parse all types then */
        while let Some(span) = spans.peek() {
            match crate::ability_tree::object::CardType::try_from_span(span) {
                Some(card_type) => {
                    match card_type.card_type {
                        mtg_data::CardType::Artifact => match result.artifact {
                            None => {
                                result.artifact = Some(ArtifactSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Battle => match result.battle {
                            None => {
                                result.battle = Some(BattleSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Conspiracy => match result.conspiracy {
                            None => {
                                result.conspiracy = Some(ConspiracySubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Creature => match result.creature {
                            None => {
                                result.creature = Some(CreatureSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Dungeon => match result.dungeon {
                            None => {
                                result.dungeon = Some(DungeonSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Emblem => match result.emblem {
                            None => {
                                result.emblem = Some(EmblemSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Enchantment => match result.enchantment {
                            None => {
                                result.enchantment = Some(EnchantmentSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Hero => match result.hero {
                            None => {
                                result.hero = Some(HeroSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Instant => match result.instant {
                            None => {
                                result.instant = Some(InstantSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Kindred => match result.kindred {
                            None => {
                                result.kindred = Some(KindredSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Land => match result.land {
                            None => {
                                result.land = Some(LandSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Phenomenon => match result.phenomenon {
                            None => {
                                result.phenomenon = Some(PhenomenonSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Plane => match result.plane {
                            None => {
                                result.plane = Some(PlaneSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Planeswalker => match result.planeswalker {
                            None => {
                                result.planeswalker = Some(PlaneswalkerSubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    loyalty: 0, /* fixme */
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Scheme => match result.scheme {
                            None => {
                                result.scheme = Some(SchemeSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Sorcery => match result.sorcery {
                            None => {
                                result.sorcery = Some(SorcerySubtype {
                                    subtypes: arrayvec::ArrayVec::new_const(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                        mtg_data::CardType::Vanguard => match result.vanguard {
                            None => {
                                result.vanguard = Some(VanguardSubtype {
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.into(),
                                })
                            }
                            Some(_) => {
                                return None;
                            }
                        },
                    }
                    let _ = spans.next();
                }
                None => break,
            }
        }

        while let Some(token) = spans.next() {
            if let Some(subtype) = &mut result.artifact {
                if let Some(new_subtype) = crate::ability_tree::object::ArtifactSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.battle {
                if let Some(new_subtype) = crate::ability_tree::object::BattleSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.creature {
                if let Some(new_subtype) = crate::ability_tree::object::CreatureSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.enchantment {
                if let Some(new_subtype) = crate::ability_tree::object::EnchantmentSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.instant {
                if let Some(new_subtype) = crate::ability_tree::object::SpellSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.kindred {
                if let Some(new_subtype) = crate::ability_tree::object::CreatureSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.land {
                if let Some(new_subtype) = crate::ability_tree::object::LandSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.planeswalker {
                if let Some(new_subtype) = crate::ability_tree::object::PlaneswalkerSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            if let Some(subtype) = &mut result.sorcery {
                if let Some(new_subtype) = crate::ability_tree::object::SpellSubtype::try_from_span(&token) {
                    if subtype.subtypes.contains(&new_subtype) {
                        return None;
                    } else {
                        subtype.subtypes.push(new_subtype);
                        continue;
                    }
                }
            }
            /* If we arrive here, no type managed to validated the given subtype, that's an error! */
            return None;
        }

        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArtifactSubtype {
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::ArtifactSubtype, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "artifact type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BattleSubtype {
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::BattleSubtype, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "battle type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConspiracySubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "conspiracy type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreatureSubtype {
    /* Fixme: power / toughness ? */
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::CreatureSubtype, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "creature type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DungeonSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "dungeon type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmblemSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "emblem type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnchantmentSubtype {
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::EnchantmentSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "enchantment type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct HeroSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "hero type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InstantSubtype {
    subtypes: arrayvec::ArrayVec<crate::ability_tree::object::SpellSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "instant type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KindredSubtype {
    subtypes: arrayvec::ArrayVec<crate::ability_tree::object::CreatureSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "kindred type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LandSubtype {
    subtypes: arrayvec::ArrayVec<crate::ability_tree::object::LandSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "land type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhenomenonSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "phenomenon type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaneSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "plane type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaneswalkerSubtype {
    pub loyalty: u64,
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::PlaneswalkerSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "planeswalker type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchemeSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "scheme type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SorcerySubtype {
    pub subtypes: arrayvec::ArrayVec<crate::ability_tree::object::SpellSubtype, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "sorcery type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VanguardSubtype {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

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

    fn node_tag(&self) -> &'static str {
        "vanguard type"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> super::span::TreeSpan {
        self.span
    }
}

/// Simplified version of the card types for meaningful card info
/// without the full card type line.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimplifiedCardTypes {
    pub artifact: bool,
    pub battle: bool,
    pub creature: bool,
    pub enchantment: bool,
    pub instant: bool,
    pub land: bool,
    pub planeswalker: bool,
    pub sorcery: bool,
}

impl From<&TypeLine> for SimplifiedCardTypes {
    fn from(type_line: &TypeLine) -> Self {
        Self {
            artifact: type_line.artifact.is_some(),
            battle: type_line.battle.is_some(),
            creature: type_line.creature.is_some(),
            enchantment: type_line.enchantment.is_some(),
            instant: type_line.instant.is_some(),
            land: type_line.land.is_some(),
            planeswalker: type_line.planeswalker.is_some(),
            sorcery: type_line.sorcery.is_some(),
        }
    }
}

impl Default for SimplifiedCardTypes {
    fn default() -> Self {
        Self {
            artifact: false,
            battle: false,
            creature: false,
            enchantment: false,
            instant: false,
            land: false,
            planeswalker: false,
            sorcery: false,
        }
    }
}
