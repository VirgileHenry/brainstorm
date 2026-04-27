use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

/// A specifier for who owns a card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerSpecifier {
    pub owner: crate::ability_tree::player::PlayerSpecifier,
    pub owned: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for OwnerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::OwnerSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.owner as &dyn AbilityTreeNode);
        children
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        unimplemented!()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "owner specifier:")?;
        out.push_final_branch()?;
        self.owner.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "owner specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnerSpecifier {
    fn dummy_init() -> Self {
        Self {
            owner: crate::utils::dummy(),
            owned: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
