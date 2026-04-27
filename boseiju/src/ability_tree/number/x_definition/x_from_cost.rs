use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An imperative for tapping an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XFromCost {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for XFromCost {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::XFromCost.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x from cost")?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x from cost"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for XFromCost {
    fn dummy_init() -> Self {
        Self {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
