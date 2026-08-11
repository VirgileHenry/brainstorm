mod characteristic_specifier;
mod subtype_specifier;

pub use characteristic_specifier::*;
pub use subtype_specifier::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for creatures.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Owner(OwnerSpecifier),
    Subtype(CreatureSubtypeSpecifier),
    WithCharacteristic(CreatureCharacteristicSpecifier),
}

impl Specifier for CreatureSpecifier {}

impl crate::Node for CreatureSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreatureSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn Node),
            Self::Color(child) => children.push(child as &dyn Node),
            Self::Control(child) => children.push(child as &dyn Node),
            Self::Owner(child) => children.push(child as &dyn Node),
            Self::Subtype(child) => children.push(child as &dyn Node),
            Self::WithCharacteristic(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
            Self::Owner(child) => child.display(out)?,
            Self::Subtype(child) => child.display(out)?,
            Self::WithCharacteristic(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Another(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::Control(child) => child.span(),
            Self::Owner(child) => child.span(),
            Self::Subtype(child) => child.span(),
            Self::WithCharacteristic(child) => child.span(),
        }
    }
}

impl Default for CreatureSpecifier {
    fn default() -> Self {
        Self::Subtype(Default::default())
    }
}
