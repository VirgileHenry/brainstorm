mod attached_to;
mod object_count;
mod object_kind;
mod object_specifiers;
mod one_among_references;
mod previously_mentionned;
mod self_referencing;
mod specified_object;

pub use attached_to::*;
pub use object_count::*;
pub use object_kind::*;
pub use object_specifiers::*;
pub use one_among_references::*;
pub use previously_mentionned::*;
pub use self_referencing::*;
pub use specified_object::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectReference {
    ObjectAttachedTo(ObjectAttachedTo),
    OneAmong(MultipleObjectReferences),
    PreviouslyMentionned(PreviouslyMentionnedObject),
    SelfReferencing(SelfReferencingObject),
    SpecifiedObj(SpecifiedObject),
}

impl crate::ability_tree::AbilityTreeNode for ObjectReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ObjectAttachedTo(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SpecifiedObj(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object reference:")?;
        out.push_final_branch()?;
        match self {
            Self::ObjectAttachedTo(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::SpecifiedObj(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ObjectAttachedTo(child) => child.node_span(),
            Self::OneAmong(child) => child.node_span(),
            Self::PreviouslyMentionned(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
            Self::SpecifiedObj(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectReference {
    fn dummy_init() -> Self {
        Self::SelfReferencing(crate::utils::dummy())
    }
}
