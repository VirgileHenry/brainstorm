use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::specified_object::SpecifiedPermanent;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardKind {
    OneAmong(OneAmong<Self>),
    Permanent(SpecifiedPermanent),
    Card {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl crate::ability_tree::AbilityTreeNode for CardKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CardKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Permanent(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Card { .. } => {
                let node_id = crate::ability_tree::NodeKind::CardBasicKind.id();
                let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card card:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::Permanent(child) => child.display(out)?,
            Self::Card { .. } => write!(out, "card")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "card card"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::OneAmong(child) => child.node_span(),
            Self::Permanent(child) => child.node_span(),
            Self::Card { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardKind {
    fn dummy_init() -> Self {
        Self::Card {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
