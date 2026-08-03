use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An action for when a creature attacks.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureAttacksAction {
    pub creature: crate::ability_tree::object::Creature,
    pub attacked_player: Option<crate::ability_tree::player::PlayerSpecifier>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreatureAttacksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureAttacksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        match self.attacked_player.as_ref() {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureAttacksAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureAttacksAction {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            attacked_player: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
