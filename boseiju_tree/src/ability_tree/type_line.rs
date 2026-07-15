use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

use serde_big_array::BigArray;
use std::ops::Add;

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
    pub span: boseiju_span::Span,
}

impl TypeLine {
    pub const FLAT_SIZE: usize = {
        <mtg_data::Supertype as idris::Idris>::COUNT
            + <mtg_data::CardType as idris::Idris>::COUNT
            + <mtg_data::ArtifactType as idris::Idris>::COUNT
            + <mtg_data::BattleType as idris::Idris>::COUNT
            + <mtg_data::CreatureType as idris::Idris>::COUNT
            + <mtg_data::EnchantmentType as idris::Idris>::COUNT
            + <mtg_data::SpellType as idris::Idris>::COUNT
            + <mtg_data::CreatureType as idris::Idris>::COUNT
            + <mtg_data::LandType as idris::Idris>::COUNT
            + <mtg_data::PlaneswalkerType as idris::Idris>::COUNT
            + <mtg_data::SpellType as idris::Idris>::COUNT
    };

    pub fn empty() -> TypeLine {
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

    pub fn flat_array(&self) -> [bool; Self::FLAT_SIZE] {
        let mut result = [false; Self::FLAT_SIZE];
        let mut offset = 0;

        let segments: &[&[bool]] = &[
            &self.supertypes,
            &self.card_types,
            &self.artifact,
            &self.battle,
            &self.creature,
            &self.enchantment,
            &self.instant,
            &self.kindred,
            &self.land,
            &self.planeswalker,
            &self.sorcery,
        ];

        for segment in segments {
            result[offset..offset + segment.len()].copy_from_slice(segment);
            offset += segment.len();
        }

        result
    }

    pub fn creature_token(
        creature_subtypes: &[mtg_data::CreatureType],
        #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
    ) -> Self {
        use idris::Idris;
        let mut result = Self::empty();
        result.supertypes[mtg_data::Supertype::Token.id()] = true;
        result.card_types[mtg_data::CardType::Creature.id()] = true;
        for subtype in creature_subtypes.iter() {
            result.creature[subtype.id()] = true;
        }
        #[cfg(feature = "spanned_tree")]
        {
            result.span = span;
        }
        result
    }

    pub fn artifact_token(
        artifact_subtypes: &[mtg_data::ArtifactType],
        #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
    ) -> Self {
        use idris::Idris;
        let mut result = Self::empty();
        result.supertypes[mtg_data::Supertype::Token.id()] = true;
        result.card_types[mtg_data::CardType::Artifact.id()] = true;
        for subtype in artifact_subtypes.iter() {
            result.artifact[subtype.id()] = true;
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

impl Node for TypeLine {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::TypeLineIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        /* Fixme: maybe type specific data shall be passed as children ? */
        arrayvec::ArrayVec::new_const()
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::TypeLine { value: self.clone() })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;

        write!(out, "type line:")?;
        out.push_final_branch()?;
        write!(out, "{self}")?;
        out.pop_branch();

        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "type line"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TypeLine {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl std::fmt::Display for TypeLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use idris::Idris;
        /* Supertypes first */
        for supertype in mtg_data::Supertype::all().filter(|s| self.supertypes[s.id()]) {
            write!(f, "{supertype} ")?;
        }
        for card_type in mtg_data::CardType::all().filter(|c| self.card_types[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::ArtifactType::all().filter(|c| self.artifact[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::BattleType::all().filter(|c| self.battle[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::CreatureType::all().filter(|c| self.creature[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::EnchantmentType::all().filter(|c| self.enchantment[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::SpellType::all().filter(|c| self.instant[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::CreatureType::all().filter(|c| self.kindred[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::LandType::all().filter(|c| self.land[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::PlaneswalkerType::all().filter(|c| self.planeswalker[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        for card_type in mtg_data::SpellType::all().filter(|c| self.sorcery[c.id()]) {
            write!(f, "{card_type} ")?;
        }
        Ok(())
    }
}

impl Default for TypeLine {
    fn default() -> Self {
        Self::empty()
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

impl Add for SimplifiedCardTypes {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            artifact: self.artifact | other.artifact,
            battle: self.battle | other.battle,
            creature: self.creature | other.creature,
            enchantment: self.enchantment | other.enchantment,
            instant: self.instant | other.instant,
            land: self.land | other.land,
            planeswalker: self.planeswalker | other.planeswalker,
            sorcery: self.sorcery | other.sorcery,
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
