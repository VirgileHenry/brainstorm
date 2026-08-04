use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An action for when a creature deals damage.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureDealsDamageAction {
    pub creature: crate::ability_tree::object::Creature,
    pub damage_kind: boseiju_lexer::terminal::DamageKind,
    pub to_player: Option<crate::ability_tree::player::PlayerReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CreatureDealsDamageAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CreatureDealsDamageAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children.push(&self.damage_kind as &dyn Node);
        match self.to_player.as_ref() {
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
        out.next_inter_branch()?;
        self.damage_kind.display(out)?;
        out.next_final_branch()?;
        match self.to_player.as_ref() {
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
        "creature blocks action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureDealsDamageAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreatureDealsDamageAction {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            damage_kind: Default::default(),
            to_player: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
