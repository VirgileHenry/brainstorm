mod artifact_specifier;

pub use artifact_specifier::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::Specifiers;

/// A specified Artifact.
///
/// This can only reference artifacts on the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifiedArtifact {
    pub specifiers: Option<Specifiers<ArtifactSpecifier>>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for SpecifiedArtifact {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpecifiedArtifact.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        match self.specifiers.as_ref() {
            Some(specifiers) => children.push(specifiers as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specified artifact:")?;
        out.push_final_branch()?;
        write!(out, "specifier(s):")?;
        out.push_final_branch()?;
        match self.specifiers.as_ref() {
            Some(specifiers) => specifiers.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specified artifact"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpecifiedArtifact {
    fn dummy_init() -> Self {
        Self {
            specifiers: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
