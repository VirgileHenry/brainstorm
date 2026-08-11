use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
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
        span: boseiju_span::Span,
    },
}

impl crate::Node for CardKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CardKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {


        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::Permanent(child) => children.push(child as &dyn Node),
            Self::Card { .. } => {
                let node_id = crate::NodeKind::CardBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::OneAmong(child) => child.span(),
            Self::Permanent(child) => child.span(),
            Self::Card { span } => *span,
        }
    }
}

impl Default for CardKind {
    fn default() -> Self {
        Self::Card {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
