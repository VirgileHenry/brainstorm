use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An action for when a creature blocks.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureBlocksAction {
    pub creature: crate::ability_tree::object::Creature,
    pub blocked_creature: Option<crate::ability_tree::object::Creature>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreatureBlocksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureBlocksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        match self.blocked_creature.as_ref() {
            Some(cost) => children.push(cost as &dyn Node),
            None => {
                let none_node = crate::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(none_node as &dyn Node);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureBlocksAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureBlocksAction {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            blocked_creature: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
