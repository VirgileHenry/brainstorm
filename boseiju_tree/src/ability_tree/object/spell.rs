use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::reference::SpellReference;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like Spells, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spell {
    OneAmong(OneAmong<Self>),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Reference(SpellReference),
}

impl crate::Node for Spell {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Spell.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn Node),
            Self::SelfReferencing(child) => children.push(child as &dyn Node),
            Self::Reference(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Reference(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Spell {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::OneAmong(child) => child.span(),
            Self::PreviouslyMentionned(child) => child.span(),
            Self::SelfReferencing(child) => child.span(),
            Self::Reference(child) => child.span(),
        }
    }
}

impl Default for Spell {
    fn default() -> Self {
        Self::Reference(Default::default())
    }
}
