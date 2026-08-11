mod any_number;
mod flat_number;
mod or_more_number;
mod up_to_number;
mod x_number;

pub mod game_state_number;
pub mod x_definition;

pub use any_number::AnyNumber;
pub use flat_number::FlatNumber;
pub use game_state_number::GameStateNumber;
pub use or_more_number::OrMoreNumber;
pub use up_to_number::UpToNumber;
pub use x_definition::XDefinition;
pub use x_number::XNumber;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A number.
///
/// A number can be as simple as a literal number (e.g. "1", "two", "4") or
/// a value that references some other objects (e.g. "ward X, where X is ...").
///
/// Number can also be "any number" where the player can choose whatever, or
/// a reference to a previosuly mentionned number in the ability.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    AnyNumber(AnyNumber),
    Flat(FlatNumber),
    OrMore(OrMoreNumber),
    UpTo(UpToNumber),
    X(XNumber),
}

impl Node for Number {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Number
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AnyNumber(child) => children.push(child as &dyn Node),
            Self::Flat(child) => children.push(child as &dyn Node),
            Self::OrMore(child) => children.push(child as &dyn Node),
            Self::UpTo(child) => children.push(child as &dyn Node),
            Self::X(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::AnyNumber(number) => number.display(out)?,
            Self::Flat(number) => number.display(out)?,
            Self::OrMore(number) => number.display(out)?,
            Self::UpTo(number) => number.display(out)?,
            Self::X(number) => number.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "number"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Number {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AnyNumber(child) => child.span(),
            Self::Flat(child) => child.span(),
            Self::OrMore(child) => child.span(),
            Self::UpTo(child) => child.span(),
            Self::X(child) => child.span(),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Flat(Default::default())
    }
}
