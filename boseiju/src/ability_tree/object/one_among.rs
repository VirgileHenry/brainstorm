use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A object reference that refers to one of multiple possible references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneAmong<T: AbilityTreeNode> {
    pub references: crate::utils::HeapArrayVec<T, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl<T: AbilityTreeNode> AbilityTreeNode for OneAmong<T> {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::OneAmong.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.references.iter() {
            children.push(specifier as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "one among:")?;
        for (i, specifier) in self.references.iter().enumerate() {
            if i == self.references.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "one among reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl<T: AbilityTreeNode> crate::utils::DummyInit for OneAmong<T> {
    fn dummy_init() -> Self {
        Self {
            references: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
