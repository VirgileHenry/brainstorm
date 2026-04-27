use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::AttachedObject;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::specified_object::SpecifiedCreature;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureReference {
    Attached(AttachedObject),
    OneAmong(OneAmong<Self>),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Specified(SpecifiedCreature),
}

impl crate::ability_tree::AbilityTreeNode for CreatureReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attached(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature reference:")?;
        out.push_final_branch()?;
        match self {
            Self::Attached(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attached(child) => child.node_span(),
            Self::OneAmong(child) => child.node_span(),
            Self::PreviouslyMentionned(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureReference {
    fn dummy_init() -> Self {
        Self::Specified(crate::utils::dummy())
    }
}
