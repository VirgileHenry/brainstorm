use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::AttachedObject;
use crate::ability_tree::object::CreatureReference;
use crate::ability_tree::object::LandReference;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::specified_object::SpecifiedPermanent;

/// An permanent reference is a way to refer specifically to permanent kinds.
/// This is useful when the action only makes sense for permanents.
///
/// For instance, an event that requires something to become tapped only makes sense
/// for permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentReference {
    Attached(AttachedObject),
    Land(LandReference),
    OneAmong(OneAmong<Self>),
    Creature(CreatureReference),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Specified(SpecifiedPermanent),
}

impl crate::ability_tree::AbilityTreeNode for PermanentReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attached(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Creature(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Land(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent reference:")?;
        out.push_final_branch()?;
        match self {
            Self::Attached(child) => child.display(out)?,
            Self::Creature(child) => child.display(out)?,
            Self::Land(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attached(child) => child.node_span(),
            Self::Creature(child) => child.node_span(),
            Self::Land(child) => child.node_span(),
            Self::OneAmong(child) => child.node_span(),
            Self::PreviouslyMentionned(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentReference {
    fn dummy_init() -> Self {
        Self::Specified(crate::utils::dummy())
    }
}
