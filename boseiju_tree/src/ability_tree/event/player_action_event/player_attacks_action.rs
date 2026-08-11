use crate::AbilityTreeNode;
use crate::MAX_CHILDREN_PER_NODE;

/// Action for a creature to deal combat damage.
///
/// Combat damage is the special kind of damage that creature deals when
/// they fight each other, or when they attack a player.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerAttacksAction {
    pub attacked_player: Option<crate::tree::player::PlayerSpecifier>,
    pub with: Option<crate::tree::object::CreatureReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl AbilityTreeNode for PlayerAttacksAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerAttacksAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        match self.attacked_player.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        match self.with.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }

        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player attacks")?;

        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player action: attack"
    }

    #[cfg(feature = "spanned_tree")]
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PlayerAttacksAction {
    fn default() -> Self {
        Self {
            attacked_player: None,
            with: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
