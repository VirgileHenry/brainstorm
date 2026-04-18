use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;
use crate::lexer::IntoToken;
use serde_big_array::BigArray;

/* Fixme: this shall be a bool array with all possible types.
 * Thanks to idris, it's super easy to do ? And changeling types MUST have all creature types this way.
 * Also, this will reduce by a bunch the max children size (this is the current cap) */

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
    pub supertypes: [bool; <mtg_data::Supertype as idris::Idris>::COUNT],
    pub card_types: [bool; <mtg_data::CardType as idris::Idris>::COUNT],
    pub artifact: [bool; <mtg_data::ArtifactType as idris::Idris>::COUNT],
    pub battle: [bool; <mtg_data::BattleType as idris::Idris>::COUNT],
    #[serde(with = "BigArray")]
    pub creature: [bool; <mtg_data::CreatureType as idris::Idris>::COUNT],
    pub enchantment: [bool; <mtg_data::EnchantmentType as idris::Idris>::COUNT],
    pub instant: [bool; <mtg_data::SpellType as idris::Idris>::COUNT],
    #[serde(with = "BigArray")]
    pub kindred: [bool; <mtg_data::CreatureType as idris::Idris>::COUNT],
    pub land: [bool; <mtg_data::LandType as idris::Idris>::COUNT],
    #[serde(with = "BigArray")]
    pub planeswalker: [bool; <mtg_data::PlaneswalkerType as idris::Idris>::COUNT],
    pub sorcery: [bool; <mtg_data::SpellType as idris::Idris>::COUNT],
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl TypeLine {
    fn empty() -> TypeLine {
        TypeLine {
            supertypes: [false; _],
            card_types: [false; _],
            artifact: [false; _],
            battle: [false; _],
            creature: [false; _],
            enchantment: [false; _],
            instant: [false; _],
            kindred: [false; _],
            land: [false; _],
            planeswalker: [false; _],
            sorcery: [false; _],
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }

    pub fn creature_token(
        creature_subtypes: &[crate::ability_tree::object::CreatureSubtype],
        #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
    ) -> Self {
        use idris::Idris;
        let mut result = Self::empty();
        result.supertypes[mtg_data::Supertype::Token.id()] = true;
        result.card_types[mtg_data::CardType::Creature.id()] = true;
        for subtype in creature_subtypes.iter() {
            result.creature[subtype.creature_subtype.id()] = true;
        }
        #[cfg(feature = "spanned_tree")]
        {
            result.span = span;
        }
        result
    }

    pub fn artifact_token(
        artifact_subtypes: &[crate::ability_tree::object::ArtifactSubtype],
        #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
    ) -> Self {
        use idris::Idris;
        let mut result = Self::empty();
        result.supertypes[mtg_data::Supertype::Token.id()] = true;
        result.card_types[mtg_data::CardType::Artifact.id()] = true;
        for subtype in artifact_subtypes.iter() {
            result.artifact[subtype.artifact_subtype.id()] = true;
        }
        #[cfg(feature = "spanned_tree")]
        {
            result.span = span;
        }
        result
    }

    pub fn card_types(&self) -> Vec<mtg_data::CardType> {
        use idris::Idris;
        mtg_data::CardType::all()
            .filter(|card_type| self.card_types[card_type.id()])
            .collect()
    }
}

impl AbilityTreeNode for TypeLine {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TypeLineIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        /* Fixme: maybe type specific data shall be passed as children ? */
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: all cards types shall be passed as data here */
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use idris::Idris;
        use std::io::Write;

        write!(out, "type line:")?;
        out.push_final_branch()?;

        for supertype in mtg_data::Supertype::all().filter(|s| self.supertypes[s.id()]) {
            write!(out, "{supertype} ")?;
        }
        for card_type in mtg_data::CardType::all().filter(|c| self.card_types[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::ArtifactType::all().filter(|c| self.artifact[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::BattleType::all().filter(|c| self.battle[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::CreatureType::all().filter(|c| self.creature[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::EnchantmentType::all().filter(|c| self.enchantment[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::SpellType::all().filter(|c| self.instant[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::CreatureType::all().filter(|c| self.kindred[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::LandType::all().filter(|c| self.land[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::PlaneswalkerType::all().filter(|c| self.planeswalker[c.id()]) {
            write!(out, "{card_type} ")?;
        }
        for card_type in mtg_data::SpellType::all().filter(|c| self.sorcery[c.id()]) {
            write!(out, "{card_type} ")?;
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
        Self::empty()
    }
}

impl IntoToken for TypeLine {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use idris::Idris;
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
                    if result.supertypes[supertype.id()] {
                        return None;
                    } else {
                        result.supertypes[supertype.id()] = true;
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
                    if result.card_types[card_type.id()] {
                        return None; /* Duplicate card type are not allowed */
                    } else {
                        result.card_types[card_type.id()] = true;
                        let _ = spans.next(); /* Token was peeked, pop it out */
                    }
                }
                None => break,
            }
        }

        /* Parse subtypes, only allow them if the associated type is found */
        while let Some(token) = spans.next() {
            if result.card_types[mtg_data::CardType::Artifact.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::ArtifactSubtype::try_from_span(&token) {
                    if result.artifact[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.artifact[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Battle.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::BattleSubtype::try_from_span(&token) {
                    if result.battle[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.battle[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Creature.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::CreatureSubtype::try_from_span(&token) {
                    if result.creature[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.creature[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Enchantment.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::EnchantmentSubtype::try_from_span(&token) {
                    if result.enchantment[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.enchantment[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Instant.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::SpellSubtype::try_from_span(&token) {
                    if result.instant[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.instant[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Kindred.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::CreatureSubtype::try_from_span(&token) {
                    if result.kindred[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.kindred[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Land.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::LandSubtype::try_from_span(&token) {
                    if result.land[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.land[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Planeswalker.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::PlaneswalkerSubtype::try_from_span(&token) {
                    if result.planeswalker[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.planeswalker[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Sorcery.id()] {
                if let Some(new_subtype) = crate::ability_tree::object::SpellSubtype::try_from_span(&token) {
                    if result.sorcery[new_subtype.id()] {
                        return None; /* Duplicates are not allowed */
                    } else {
                        result.sorcery[new_subtype.id()] = true;
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
        use idris::Idris;
        Self {
            artifact: type_line.card_types[mtg_data::CardType::Artifact.id()],
            battle: type_line.card_types[mtg_data::CardType::Battle.id()],
            creature: type_line.card_types[mtg_data::CardType::Creature.id()],
            enchantment: type_line.card_types[mtg_data::CardType::Enchantment.id()],
            instant: type_line.card_types[mtg_data::CardType::Instant.id()],
            land: type_line.card_types[mtg_data::CardType::Land.id()],
            planeswalker: type_line.card_types[mtg_data::CardType::Planeswalker.id()],
            sorcery: type_line.card_types[mtg_data::CardType::Sorcery.id()],
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
