use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

/// A specifier for who controls a permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSpecifier {
    pub controller: crate::ability_tree::player::PlayerSpecifier,
    pub controlled: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ControlSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ControlSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.controller as &dyn AbilityTreeNode);
        children
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "control specifier:")?;
        out.push_final_branch()?;
        self.controller.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "control specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ControlSpecifier {
    fn dummy_init() -> Self {
        Self {
            controller: crate::utils::dummy(),
            controlled: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
