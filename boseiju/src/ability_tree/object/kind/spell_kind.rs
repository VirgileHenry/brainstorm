use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::SelfReferencing;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellKind {
    OneAmong(OneAmong<Self>),
    SelfReferencing(SelfReferencing),
}

impl crate::ability_tree::AbilityTreeNode for SpellKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpellKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell reference:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::OneAmong(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpellKind {
    fn dummy_init() -> Self {
        Self::SelfReferencing(crate::utils::dummy())
    }
}
