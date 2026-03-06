mod power_toughness_modifiers;

pub use power_toughness_modifiers::PowerToughnessModifiers;
pub use power_toughness_modifiers::PowerToughnessModifiersMinusMinus;
pub use power_toughness_modifiers::PowerToughnessModifiersMinusPlus;
pub use power_toughness_modifiers::PowerToughnessModifiersPlusMinus;
pub use power_toughness_modifiers::PowerToughnessModifiersPlusPlus;
pub use power_toughness_modifiers::PowerToughnessModifiersSet;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A modification to the characteristics of an object.
///
/// The characteristics are Name, mana cost, color, card type, subtype,
/// supertype, rules text, abilities, power and toughness, loyalty,
/// hand modifier and life modifier (the last two are for vanguard only).
///
/// See also <https://mtg.fandom.com/wiki/Object>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectCharacteristicModification {
    PowerToughnessModifiers(PowerToughnessModifiers),
}

#[cfg(feature = "spanned_tree")]
impl ObjectCharacteristicModification {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::PowerToughnessModifiers(child) => child.node_span(),
        }
    }
}

impl AbilityTreeNode for ObjectCharacteristicModification {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectCharacteristicModification.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::PowerToughnessModifiers(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object characteristic modification:")?;
        out.push_final_branch()?;
        match self {
            Self::PowerToughnessModifiers(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object characteristics modification"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::PowerToughnessModifiers(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectCharacteristicModification {
    fn dummy_init() -> Self {
        Self::PowerToughnessModifiers(crate::utils::dummy())
    }
}
