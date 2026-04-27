use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::tree_node::MtgDataNodeKind;
use crate::ability_tree::tree_node::NodeKind;
use idris::Idris;

/// List of all card types that can be permanents.
///
/// This is a subset of the card type, and therefore methods that require
/// a type id will yield the same children as the card type ones, for consistency.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermanentKind {
    Artifact {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Battle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Creature {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enchantment {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Land {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Planeswalker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl PermanentKind {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Artifact {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Battle {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Creature {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Enchantment {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Land {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Planeswalker {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }

    pub fn to_card_type(self) -> mtg_data::CardType {
        match self {
            Self::Artifact { .. } => mtg_data::CardType::Artifact,
            Self::Battle { .. } => mtg_data::CardType::Battle,
            Self::Creature { .. } => mtg_data::CardType::Creature,
            Self::Enchantment { .. } => mtg_data::CardType::Enchantment,
            Self::Land { .. } => mtg_data::CardType::Land,
            Self::Planeswalker { .. } => mtg_data::CardType::Planeswalker,
        }
    }

    pub fn try_from_card_type(
        card_type: mtg_data::CardType,
        #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
    ) -> Result<Self, &'static str> {
        match card_type {
            mtg_data::CardType::Artifact => Ok(Self::Artifact {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            mtg_data::CardType::Battle => Ok(Self::Battle {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            mtg_data::CardType::Creature => Ok(Self::Creature {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            mtg_data::CardType::Enchantment => Ok(Self::Enchantment {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            mtg_data::CardType::Land => Ok(Self::Land {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            mtg_data::CardType::Planeswalker => Ok(Self::Planeswalker {
                #[cfg(feature = "spanned_tree")]
                span,
            }),
            _ => Err("Card type is not a permanent"),
        }
    }
}

impl AbilityTreeNode for PermanentKind {
    fn node_id(&self) -> usize {
        crate::ability_tree::NodeKind::MtgData(MtgDataNodeKind::PermanentKindIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        /* Since all permanent types are card types, use the card types for consistency. */
        let child_id = NodeKind::MtgData(MtgDataNodeKind::CardType(self.to_card_type())).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{}", self.to_card_type())
    }

    fn node_tag(&self) -> &'static str {
        "permanent kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Artifact { span } => *span,
            Self::Battle { span } => *span,
            Self::Creature { span } => *span,
            Self::Enchantment { span } => *span,
            Self::Land { span } => *span,
            Self::Planeswalker { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentKind {
    fn dummy_init() -> Self {
        Self::Creature {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
