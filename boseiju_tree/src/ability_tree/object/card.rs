mod top_cards_of_libary;

pub use top_cards_of_libary::TopCardsOfLibrary;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::AttachedObject;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::reference::CardReference;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Card {
    Attached(AttachedObject),
    OneAmong(OneAmong<Self>),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Reference(CardReference),
    TopCardsOfLibrary(TopCardsOfLibrary),
}

impl crate::Node for Card {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Card.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attached(child) => children.push(child as &dyn Node),
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn Node),
            Self::SelfReferencing(child) => children.push(child as &dyn Node),
            Self::Reference(child) => children.push(child as &dyn Node),
            Self::TopCardsOfLibrary(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card:")?;
        out.push_final_branch()?;
        match self {
            Self::Attached(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Reference(child) => child.display(out)?,
            Self::TopCardsOfLibrary(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Card {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Attached(child) => child.span(),
            Self::OneAmong(child) => child.span(),
            Self::PreviouslyMentionned(child) => child.span(),
            Self::SelfReferencing(child) => child.span(),
            Self::Reference(child) => child.span(),
            Self::TopCardsOfLibrary(child) => child.span(),
        }
    }
}


impl Default for Card {
    fn default() -> Self {
        Self::Reference(Default::default())
    }
}
