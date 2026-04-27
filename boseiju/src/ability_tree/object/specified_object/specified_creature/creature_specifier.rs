mod characteristic_specifier;
mod subtype_specifier;

pub use characteristic_specifier::*;
pub use subtype_specifier::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for creatures.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Subtype(CreatureSubtypeSpecifier),
    WithCharacteristic(CreatureCharacteristicSpecifier),
}

impl Specifier for CreatureSpecifier {}

impl crate::ability_tree::AbilityTreeNode for CreatureSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Subtype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::WithCharacteristic(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
            Self::Subtype(child) => child.display(out)?,
            Self::WithCharacteristic(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.node_span(),
            Self::Color(child) => child.node_span(),
            Self::Control(child) => child.node_span(),
            Self::Subtype(child) => child.node_span(),
            Self::WithCharacteristic(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureSpecifier {
    fn dummy_init() -> Self {
        Self::Subtype(crate::utils::dummy())
    }
}
