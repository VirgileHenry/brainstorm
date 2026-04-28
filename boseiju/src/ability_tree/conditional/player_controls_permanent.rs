use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A condition that is met when a given event has occured in a given timeframe.
///
/// Examples are, "if you attacked this turn" or "if a creature died this turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerControlsPermanent {
    pub player: crate::ability_tree::player::PlayerSpecifier,
    pub permanent: crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PlayerControlsPermanent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerControlsPermanent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        children.push(&self.permanent as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player controls permanent:")?;
        out.push_inter_branch()?;
        self.player.display(out)?;
        out.next_final_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player controls permanent"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerControlsPermanent {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            permanent: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
