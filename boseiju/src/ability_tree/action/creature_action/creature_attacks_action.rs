use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An action for when a creature attacks.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureAttacksAction {
    pub creature: crate::ability_tree::object::Creature,
    pub attacked_player: Option<crate::ability_tree::player::PlayerSpecifier>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CreatureAttacksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureAttacksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn AbilityTreeNode);
        match self.attacked_player.as_ref() {
            Some(cost) => children.push(cost as &dyn AbilityTreeNode),
            None => {
                let none_node = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(none_node as &dyn AbilityTreeNode);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature attacks action:")?;
        out.push_inter_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        match self.attacked_player.as_ref() {
            Some(player) => {
                write!(out, "attacked player:")?;
                out.push_final_branch()?;
                player.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "any player")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature attacks action"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureAttacksAction {
    fn dummy_init() -> Self {
        Self {
            creature: crate::utils::dummy(),
            attacked_player: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
