mod power_toughness_modifiers;

pub use power_toughness_modifiers::PowerToughnessModifiers;
pub use power_toughness_modifiers::PowerToughnessModifiersMinusMinus;
pub use power_toughness_modifiers::PowerToughnessModifiersMinusPlus;
pub use power_toughness_modifiers::PowerToughnessModifiersPlusMinus;
pub use power_toughness_modifiers::PowerToughnessModifiersPlusPlus;
pub use power_toughness_modifiers::PowerToughnessModifiersSet;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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

impl Node for ObjectCharacteristicModification {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ObjectCharacteristicModification.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::PowerToughnessModifiers(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectCharacteristicModification {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::PowerToughnessModifiers(child) => child.span(),
        }
    }
}

impl Default for ObjectCharacteristicModification {
    fn default() -> Self {
        Self::PowerToughnessModifiers(Default::default())
    }
}
