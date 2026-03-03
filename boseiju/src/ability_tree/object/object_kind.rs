mod artifact_subtype;
mod battle_subtype;
mod card_type;
mod creature_subtype;
mod enchantment_subtype;
mod instant_sorcery_subtype;
mod land_subtype;
mod planeswalker_subtype;
mod supertype;

pub use artifact_subtype::ArtifactSubtype;
pub use battle_subtype::BattleSubtype;
pub use card_type::CardType;
pub use creature_subtype::CreatureSubtype;
pub use enchantment_subtype::EnchantmentSubtype;
pub use instant_sorcery_subtype::SpellSubtype;
pub use land_subtype::LandSubtype;
pub use planeswalker_subtype::PlaneswalkerSubtype;
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
    Card {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CreatureSubtype(CreatureSubtype),
    EnchantmentSubtype(EnchantmentSubtype),
    InstantSorcerySubtype(SpellSubtype),
    LandSubtype(LandSubtype),
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlaneswalkerSubtype(PlaneswalkerSubtype),
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Supertype(Supertype),
    CardType(CardType),
}

impl ObjectKind {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Card {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Spell {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
        .chain(ArtifactSubtype::all().map(Self::ArtifactSubtype))
        .chain(BattleSubtype::all().map(Self::BattleSubtype))
        .chain(CardType::all().map(Self::CardType))
        .chain(CreatureSubtype::all().map(Self::CreatureSubtype))
        .chain(EnchantmentSubtype::all().map(Self::EnchantmentSubtype))
        .chain(SpellSubtype::all().map(Self::InstantSorcerySubtype))
        .chain(LandSubtype::all().map(Self::LandSubtype))
        .chain(PlaneswalkerSubtype::all().map(Self::PlaneswalkerSubtype))
        .chain(Supertype::all().map(Self::Supertype))
    }

    #[cfg(feature = "spanned_tree")]
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Card { span } => *span,
            Self::Permanent { span } => *span,
            Self::Spell { span } => *span,
            Self::ArtifactSubtype(child) => child.span,
            Self::BattleSubtype(child) => child.span,
            Self::CreatureSubtype(child) => child.span,
            Self::EnchantmentSubtype(child) => child.span,
            Self::InstantSorcerySubtype(child) => child.span,
            Self::LandSubtype(child) => child.span,
            Self::PlaneswalkerSubtype(child) => child.span,
            Self::Supertype(child) => child.span,
            Self::CardType(child) => child.span,
        }
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
            Self::Card { .. } => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::CardIdMarker).id(),
            ) as &dyn AbilityTreeNode),
            Self::Permanent { .. } => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::PermanentIdMarker).id(),
            ) as &dyn AbilityTreeNode),
            Self::Spell { .. } => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
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
        write!(out, "object kind")?;
        out.push_final_branch()?;
        match self {
            Self::Card { .. } => write!(out, "card")?,
            Self::Permanent { .. } => write!(out, "permanent")?,
            Self::Spell { .. } => write!(out, "spell")?,
            Self::ArtifactSubtype(child) => child.display(out)?,
            Self::BattleSubtype(child) => child.display(out)?,
            Self::CreatureSubtype(child) => child.display(out)?,
            Self::EnchantmentSubtype(child) => child.display(out)?,
            Self::InstantSorcerySubtype(child) => child.display(out)?,
            Self::LandSubtype(child) => child.display(out)?,
            Self::PlaneswalkerSubtype(child) => child.display(out)?,
            Self::Supertype(child) => child.display(out)?,
            Self::CardType(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for ObjectKind {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        if let Some(subtype) = ArtifactSubtype::try_from_span(span) {
            return Some(Self::ArtifactSubtype(subtype));
        } else if let Some(subtype) = BattleSubtype::try_from_span(span) {
            return Some(Self::BattleSubtype(subtype));
        } else if let Some(subtype) = CardType::try_from_span(span) {
            return Some(Self::CardType(subtype));
        } else if let Some(subtype) = CreatureSubtype::try_from_span(span) {
            return Some(Self::CreatureSubtype(subtype));
        } else if let Some(subtype) = EnchantmentSubtype::try_from_span(span) {
            return Some(Self::EnchantmentSubtype(subtype));
        } else if let Some(subtype) = SpellSubtype::try_from_span(span) {
            return Some(Self::InstantSorcerySubtype(subtype));
        } else if let Some(subtype) = LandSubtype::try_from_span(span) {
            return Some(Self::LandSubtype(subtype));
        } else if let Some(subtype) = PlaneswalkerSubtype::try_from_span(span) {
            return Some(Self::PlaneswalkerSubtype(subtype));
        } else if let Some(subtype) = Supertype::try_from_span(span) {
            return Some(Self::Supertype(subtype));
        } else {
            match span.text {
                "card" | "cards" => Some(Self::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "permanent" | "permanents" => Some(Self::Permanent {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "spell" | "spells" => Some(Self::Spell {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                _ => None,
            }
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectKind {
    fn dummy_init() -> Self {
        Self::Card {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
