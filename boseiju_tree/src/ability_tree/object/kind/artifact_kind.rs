use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactKind {
    Artifact {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span
    },
}

impl crate::Node for ArtifactKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ArtifactKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Artifact { .. } => {
                let node_id = crate::NodeKind::ArtifactBasicKind.id();
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "artifact kind:")?;
        out.push_final_branch()?;
        match self {
            Self::Artifact { .. } => write!(out, "artifact")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "artifact kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ArtifactKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Artifact { span } => *span,
        }
    }
}


impl Default for ArtifactKind {
    fn default() -> Self {
        Self::Artifact {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
