use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::kind::PermanentKind;
use crate::ability_tree::object::specified_object::SpecifiedCard;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardKind {
    OneAmong(OneAmong<Self>),
    Permanent(PermanentKind),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Specified(SpecifiedCard),
}

impl crate::ability_tree::AbilityTreeNode for CardKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Permanent(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card reference:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::Permanent(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::OneAmong(child) => child.node_span(),
            Self::Permanent(child) => child.node_span(),
            Self::PreviouslyMentionned(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardKind {
    fn dummy_init() -> Self {
        Self::SelfReferencing(crate::utils::dummy())
    }
}
