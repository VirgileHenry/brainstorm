mod artifact_subtype;
mod battle_subtype;
mod card;
mod card_type;
mod creature_subtype;
mod enchantment_subtype;
mod instant_sorcery_subtype;
mod land_subtype;
mod permanent;
mod planeswalker_subtype;
mod spell;
mod supertype;

pub use artifact_subtype::ArtifactSubtype;
pub use battle_subtype::BattleSubtype;
pub use card::CardObjectKind;
pub use card_type::CardType;
pub use creature_subtype::CreatureSubtype;
pub use enchantment_subtype::EnchantmentSubtype;
pub use instant_sorcery_subtype::SpellSubtype;
pub use land_subtype::LandSubtype;
pub use permanent::PermanentObjectKind;
pub use planeswalker_subtype::PlaneswalkerSubtype;
pub use spell::SpellObjectKind;
pub use supertype::Supertype;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An object kind wraps up most of the supertypes, types and subtypes,
/// as well as some grouping types, such as spell, card and permanent.
///
/// It is mostly made to be an object specifier (e.g. "destroy target creature /
/// enchantment / permanent").
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectKind {
    ArtifactSubtype(ArtifactSubtype),
    BattleSubtype(BattleSubtype),
    Card(CardObjectKind),
    CardType(CardType),
    CreatureSubtype(CreatureSubtype),
    EnchantmentSubtype(EnchantmentSubtype),
    InstantSorcerySubtype(SpellSubtype),
    LandSubtype(LandSubtype),
    Permanent(PermanentObjectKind),
    PlaneswalkerSubtype(PlaneswalkerSubtype),
    Spell(SpellObjectKind),
    Supertype(Supertype),
}

impl ObjectKind {
    pub fn all() -> impl Iterator<Item = Self> {
        std::iter::empty()
            .chain(ArtifactSubtype::all().map(Self::ArtifactSubtype))
            .chain(BattleSubtype::all().map(Self::BattleSubtype))
            .chain(CardObjectKind::all().map(Self::Card))
            .chain(CardType::all().map(Self::CardType))
            .chain(CreatureSubtype::all().map(Self::CreatureSubtype))
            .chain(EnchantmentSubtype::all().map(Self::EnchantmentSubtype))
            .chain(SpellSubtype::all().map(Self::InstantSorcerySubtype))
            .chain(LandSubtype::all().map(Self::LandSubtype))
            .chain(PermanentObjectKind::all().map(Self::Permanent))
            .chain(PlaneswalkerSubtype::all().map(Self::PlaneswalkerSubtype))
            .chain(SpellObjectKind::all().map(Self::Spell))
            .chain(Supertype::all().map(Self::Supertype))
    }
}

impl AbilityTreeNode for ObjectKind {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::MtgDataNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::ObjectKindIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ArtifactSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::BattleSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Card(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardType(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CreatureSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::EnchantmentSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::InstantSorcerySubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::LandSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Permanent(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PlaneswalkerSubtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Spell(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Supertype(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object kind")?;
        out.push_final_branch()?;
        match self {
            Self::ArtifactSubtype(child) => child.display(out)?,
            Self::BattleSubtype(child) => child.display(out)?,
            Self::Card(child) => child.display(out)?,
            Self::CardType(child) => child.display(out)?,
            Self::CreatureSubtype(child) => child.display(out)?,
            Self::EnchantmentSubtype(child) => child.display(out)?,
            Self::InstantSorcerySubtype(child) => child.display(out)?,
            Self::LandSubtype(child) => child.display(out)?,
            Self::Permanent(child) => child.display(out)?,
            Self::PlaneswalkerSubtype(child) => child.display(out)?,
            Self::Spell(child) => child.display(out)?,
            Self::Supertype(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ArtifactSubtype(child) => child.node_span(),
            Self::BattleSubtype(child) => child.node_span(),
            Self::Card(child) => child.node_span(),
            Self::CardType(child) => child.node_span(),
            Self::CreatureSubtype(child) => child.node_span(),
            Self::EnchantmentSubtype(child) => child.node_span(),
            Self::InstantSorcerySubtype(child) => child.node_span(),
            Self::LandSubtype(child) => child.node_span(),
            Self::Permanent(child) => child.node_span(),
            Self::PlaneswalkerSubtype(child) => child.node_span(),
            Self::Spell(child) => child.node_span(),
            Self::Supertype(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for ObjectKind {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        if let Some(result) = ArtifactSubtype::try_from_span(span) {
            Some(Self::ArtifactSubtype(result))
        } else if let Some(result) = BattleSubtype::try_from_span(span) {
            Some(Self::BattleSubtype(result))
        } else if let Some(result) = CardObjectKind::try_from_span(span) {
            Some(Self::Card(result))
        } else if let Some(result) = CardType::try_from_span(span) {
            Some(Self::CardType(result))
        } else if let Some(result) = CreatureSubtype::try_from_span(span) {
            Some(Self::CreatureSubtype(result))
        } else if let Some(result) = EnchantmentSubtype::try_from_span(span) {
            Some(Self::EnchantmentSubtype(result))
        } else if let Some(result) = SpellSubtype::try_from_span(span) {
            Some(Self::InstantSorcerySubtype(result))
        } else if let Some(result) = LandSubtype::try_from_span(span) {
            Some(Self::LandSubtype(result))
        } else if let Some(result) = PermanentObjectKind::try_from_span(span) {
            Some(Self::Permanent(result))
        } else if let Some(result) = PlaneswalkerSubtype::try_from_span(span) {
            Some(Self::PlaneswalkerSubtype(result))
        } else if let Some(result) = SpellObjectKind::try_from_span(span) {
            Some(Self::Spell(result))
        } else if let Some(result) = Supertype::try_from_span(span) {
            Some(Self::Supertype(result))
        } else {
            None
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectKind {
    fn dummy_init() -> Self {
        Self::Card(crate::utils::dummy())
    }
}
