use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureKind {
    Creature {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl crate::Node for CreatureKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreatureKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Creature { .. } => {
                let node_id = crate::NodeKind::CreatureBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::Creature { .. } => write!(out, "creature")?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Creature { span } => *span,
        }
    }
}

impl Default for CreatureKind {
    fn default() -> Self {
        Self::Creature {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
