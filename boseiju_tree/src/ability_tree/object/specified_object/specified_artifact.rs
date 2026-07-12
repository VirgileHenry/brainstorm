mod artifact_specifier;

pub use artifact_specifier::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::kind::ArtifactKind;
use crate::ability_tree::object::specified_object::Specifiers;

/// A specified Artifact.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifiedArtifact {
    pub kind: ArtifactKind,
    pub specifiers: Option<Specifiers<ArtifactSpecifier>>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl SpecifiedArtifact {
    pub fn add_factor_specifier(&self, factor_specifier: ArtifactSpecifier) -> Self {
        #[cfg(feature = "spanned_tree")]
        let factor_specifier_span = {
            use boseiju_span::Spanned;
            factor_specifier.span()
        };
        match &self.specifiers {
            Some(prev_specifiers) => SpecifiedArtifact {
                kind: self.kind.clone(),
                specifiers: Some(prev_specifiers.add_factor_specifier(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
            None => SpecifiedArtifact {
                kind: self.kind.clone(),
                specifiers: Some(Specifiers::Single(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
        }
    }
}

impl Node for SpecifiedArtifact {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpecifiedArtifact.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        match self.specifiers.as_ref() {
            Some(specifiers) => children.push(specifiers as &dyn Node),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specified artifact:")?;
        out.push_inter_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpecifiedArtifact {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpecifiedArtifact {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            specifiers: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
