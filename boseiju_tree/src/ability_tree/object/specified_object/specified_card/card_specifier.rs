mod characteristic_specifier;

pub use characteristic_specifier::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
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

impl crate::Node for CardSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CardSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn Node),
            Self::Color(child) => children.push(child as &dyn Node),
            Self::Owner(child) => children.push(child as &dyn Node),
            Self::WithCharacteristic(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
impl boseiju_span::Spanned for CardSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Another(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::Owner(child) => child.span(),
            Self::WithCharacteristic(child) => child.span(),
        }
    }
}

impl Default for CardSpecifier {
    fn default() -> Self {
        Self::Color(Default::default())
    }
}
