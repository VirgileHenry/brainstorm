use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An action for when a creature blocks.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureBlocksAction {
    pub creature: crate::ability_tree::object::Creature,
    pub blocked_creature: Option<crate::ability_tree::object::Creature>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CreatureBlocksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureBlocksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn AbilityTreeNode);
        match self.blocked_creature.as_ref() {
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
        write!(out, "creature blocks action:")?;
        out.push_inter_branch()?;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        self.creature.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        match self.blocked_creature.as_ref() {
            Some(player) => {
                write!(out, "blocked creature:")?;
                out.push_final_branch()?;
                player.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "any creature")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature blocks action"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureBlocksAction {
    fn dummy_init() -> Self {
        Self {
            creature: crate::utils::dummy(),
            blocked_creature: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
