mod characteristic_specifier;

pub use characteristic_specifier::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for cards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Owner(OwnerSpecifier),
    WithCharacteristic(CardCharacteristicSpecifier),
}

impl Specifier for CardSpecifier {}

impl crate::ability_tree::AbilityTreeNode for CardSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Owner(child) => children.push(child as &dyn AbilityTreeNode),
            Self::WithCharacteristic(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Owner(child) => child.display(out)?,
            Self::WithCharacteristic(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for CardSpecifier {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::Owner(child) => child.span(),
            Self::WithCharacteristic(child) => child.span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardSpecifier {
    fn dummy_init() -> Self {
        Self::Color(crate::utils::dummy())
    }
}
