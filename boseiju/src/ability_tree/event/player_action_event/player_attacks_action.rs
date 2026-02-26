use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Action for a creature to deal combat damage.
///
/// Combat damage is the special kind of damage that creature deals when
/// they fight each other, or when they attack a player.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlayerAttacksAction {
    pub attacked_player: Option<crate::ability_tree::terminals::PlayerSpecifier>,
    pub with: Option<crate::ability_tree::object::ObjectReference>,
}

impl AbilityTreeNode for PlayerAttacksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerAttacksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

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

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player attacks")?;

        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerAttacksAction {
    fn dummy_init() -> Self {
        Self {
            attacked_player: None,
            with: None,
        }
    }
}
