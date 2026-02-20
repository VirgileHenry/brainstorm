use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An object kind wraps up most of the supertypes, types and subtypes,
/// as well as some grouping types, such as spell, card and permanent.
///
/// It is mostly made to be an object specifier (e.g. "destroy target creature /
/// enchantment / permanent").
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectKind {
    ArtifactSubtype(mtg_data::ArtifactType),
    BattleSubtype(mtg_data::BattleType),
    Card,
    CreatureSubtype(mtg_data::CreatureType),
    EnchantmentSubtype(mtg_data::EnchantmentType),
    InstantSorcerySubtype(mtg_data::SpellType),
    LandSubtype(mtg_data::LandType),
    Permanent,
    PlaneswalkerSubtype(mtg_data::PlaneswalkerType),
    Spell,
    Supertype(mtg_data::Supertype),
    CardType(mtg_data::CardType),
}

impl ObjectKind {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Card, Self::Permanent, Self::Spell]
            .into_iter()
            .chain(mtg_data::ArtifactType::all().map(Self::ArtifactSubtype))
            .chain(mtg_data::BattleType::all().map(Self::BattleSubtype))
            .chain(mtg_data::CreatureType::all().map(Self::CreatureSubtype))
            .chain(mtg_data::EnchantmentType::all().map(Self::EnchantmentSubtype))
            .chain(mtg_data::SpellType::all().map(Self::InstantSorcerySubtype))
            .chain(mtg_data::LandType::all().map(Self::LandSubtype))
            .chain(mtg_data::PlaneswalkerType::all().map(Self::PlaneswalkerSubtype))
            .chain(mtg_data::Supertype::all().map(Self::Supertype))
            .chain(mtg_data::CardType::all().map(Self::CardType))
    }
}

impl AbilityTreeNode for ObjectKind {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ObjectKindIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Card => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::CardIdMarker).id(),
            ) as &dyn AbilityTreeNode),
            Self::Permanent => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::PermanentIdMarker).id(),
            ) as &dyn AbilityTreeNode),
            Self::Spell => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::SpellIdMarker).id(),
            ) as &dyn AbilityTreeNode),
            Self::ArtifactSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::BattleSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CreatureSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::EnchantmentSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::InstantSorcerySubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::LandSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlaneswalkerSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Supertype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardType(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArtifactSubtype(subtype) => subtype.fmt(f),
            Self::BattleSubtype(subtype) => subtype.fmt(f),
            Self::Card => write!(f, "card"),
            Self::CreatureSubtype(subtype) => subtype.fmt(f),
            Self::EnchantmentSubtype(subtype) => subtype.fmt(f),
            Self::InstantSorcerySubtype(subtype) => subtype.fmt(f),
            Self::LandSubtype(subtype) => subtype.fmt(f),
            Self::Permanent => write!(f, "permanent"),
            Self::PlaneswalkerSubtype(subtype) => subtype.fmt(f),
            Self::Spell => write!(f, "spell"),
            Self::Supertype(supertype) => supertype.fmt(f),
            Self::CardType(ty) => ty.fmt(f),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for ObjectKind {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        if let Some(subtype) = mtg_data::ArtifactType::try_from_str(source) {
            return Some(Self::ArtifactSubtype(subtype));
        } else if let Some(subtype) = mtg_data::BattleType::try_from_str(source) {
            return Some(Self::BattleSubtype(subtype));
        } else if let Some(subtype) = mtg_data::CreatureType::try_from_str(source) {
            return Some(Self::CreatureSubtype(subtype));
        } else if let Some(subtype) = mtg_data::EnchantmentType::try_from_str(source) {
            return Some(Self::EnchantmentSubtype(subtype));
        } else if let Some(subtype) = mtg_data::SpellType::try_from_str(source) {
            return Some(Self::InstantSorcerySubtype(subtype));
        } else if let Some(subtype) = mtg_data::LandType::try_from_str(source) {
            return Some(Self::LandSubtype(subtype));
        } else if let Some(subtype) = mtg_data::PlaneswalkerType::try_from_str(source) {
            return Some(Self::PlaneswalkerSubtype(subtype));
        } else if let Some(subtype) = mtg_data::Supertype::try_from_str(source) {
            return Some(Self::Supertype(subtype));
        } else if let Some(subtype) = mtg_data::CardType::try_from_str(source) {
            return Some(Self::CardType(subtype));
        } else {
            match source {
                "card" | "cards" => Some(Self::Card),
                "permanent" | "permanents" => Some(Self::Permanent),
                "spell" | "spells" => Some(Self::Spell),
                _ => None,
            }
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectKind {
    fn dummy_init() -> Self {
        Self::Card
    }
}
