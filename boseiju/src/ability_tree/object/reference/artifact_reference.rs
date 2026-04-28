use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A Artifact reference.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactReference {
    pub count: crate::ability_tree::object::CountSpecifier,
    pub kind: crate::ability_tree::object::kind::ArtifactKind,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ArtifactReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ArtifactReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn AbilityTreeNode);
        children.push(&self.kind as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Artifact reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "Artifact reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ArtifactReference {
    fn dummy_init() -> Self {
        Self {
            count: crate::utils::dummy(),
            kind: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
