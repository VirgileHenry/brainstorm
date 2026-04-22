mod mana_value;
mod power;
mod total_mana_value;
mod total_power;
mod total_power_and_toughness;
mod toughness;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

pub use mana_value::CardManaValuePropertySpecifier;
pub use power::CardPowerPropertySpecifier;
pub use total_mana_value::CardTotalManaValuePropertySpecifier;
pub use total_power::CardTotalPowerPropertySpecifier;
pub use total_power_and_toughness::CardTotalPowerAndToughnessPropertySpecifier;
pub use toughness::CardToughnessPropertySpecifier;

/// A specifier for a property of the card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardPropertySpecifier {
    CardManaValue(CardManaValuePropertySpecifier),
    CardPower(CardPowerPropertySpecifier),
    CardTotalManaValue(CardTotalManaValuePropertySpecifier),
    CardTotalPower(CardTotalPowerPropertySpecifier),
    CardTotalPowerAndToughness(CardTotalPowerAndToughnessPropertySpecifier),
    CardToughness(CardToughnessPropertySpecifier),
}

impl AbilityTreeNode for CardPropertySpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardPropertySpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CardManaValue(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardPower(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardTotalManaValue(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardTotalPower(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardTotalPowerAndToughness(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CardToughness(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card property specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::CardManaValue(child) => child.display(out)?,
            Self::CardPower(child) => child.display(out)?,
            Self::CardTotalManaValue(child) => child.display(out)?,
            Self::CardTotalPower(child) => child.display(out)?,
            Self::CardTotalPowerAndToughness(child) => child.display(out)?,
            Self::CardToughness(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card property specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CardManaValue(child) => child.node_span(),
            Self::CardPower(child) => child.node_span(),
            Self::CardTotalManaValue(child) => child.node_span(),
            Self::CardTotalPower(child) => child.node_span(),
            Self::CardTotalPowerAndToughness(child) => child.node_span(),
            Self::CardToughness(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardPropertySpecifier {
    fn dummy_init() -> Self {
        Self::CardPower(crate::utils::dummy())
    }
}
