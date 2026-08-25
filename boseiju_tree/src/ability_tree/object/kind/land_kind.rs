use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandKind {
    Land {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl crate::Node for LandKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::LandKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Land { .. } => {
                let node_id = crate::NodeKind::LandBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::Land { .. } => write!(out, "land")?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "land kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for LandKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Land { span } => *span,
        }
    }
}

impl Default for LandKind {
    fn default() -> Self {
        Self::Land {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
