mod attached_to;
mod object_kind;
mod object_specifiers;
mod previously_mentionned;
mod self_referencing;
mod specified_object;

pub use attached_to::ObjectAttachedTo;
pub use object_kind::ObjectKind;
pub use object_specifiers::AnotherObjectSpecifier;
pub use object_specifiers::ObjectSpecifier;
pub use object_specifiers::ObjectSpecifiers;
pub use object_specifiers::SpecifierAndList;
pub use object_specifiers::SpecifierOrList;
pub use object_specifiers::SpecifierOrOfAndList;
pub use previously_mentionned::PreviouslyMentionnedObject;
pub use self_referencing::SelfReferencingObject;
pub use specified_object::SpecifiedObject;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectReference {
    SelfReferencing(SelfReferencingObject),
    ObjectAttachedTo(ObjectAttachedTo),
    SpecifiedObj(SpecifiedObject),
    PreviouslyMentionned(PreviouslyMentionnedObject),
}

impl crate::ability_tree::AbilityTreeNode for ObjectReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        match self {
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::ObjectAttachedTo(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SpecifiedObj(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object reference:")?;
        out.push_final_branch()?;
        match self {
            Self::SelfReferencing(child) => child.display(out)?,
            Self::ObjectAttachedTo(child) => child.display(out)?,
            Self::SpecifiedObj(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectReference {
    fn dummy_init() -> Self {
        Self::SelfReferencing(crate::utils::dummy())
    }
}
