use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A specifier for who casts a spell.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CastSpecifier {
    pub casting_player: crate::ability_tree::player::PlayerSpecifier,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for CastSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CastSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.casting_player as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "caster specifier:")?;
        out.push_final_branch()?;
        self.casting_player.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "caster specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CastSpecifier {
    fn dummy_init() -> Self {
        Self {
            casting_player: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
