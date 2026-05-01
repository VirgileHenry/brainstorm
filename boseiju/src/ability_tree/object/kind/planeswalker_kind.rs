use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaneswalkerKind {
    Planeswalker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl crate::ability_tree::AbilityTreeNode for PlaneswalkerKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlaneswalkerKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Planeswalker { .. } => {
                let node_id = crate::ability_tree::NodeKind::PlaneswalkerBasicKind.id();
                let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "planeswalker kind:")?;
        out.push_final_branch()?;
        match self {
            Self::Planeswalker { .. } => write!(out, "planeswalker")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "planeswalker kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Planeswalker { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlaneswalkerKind {
    fn dummy_init() -> Self {
        Self::Planeswalker {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
