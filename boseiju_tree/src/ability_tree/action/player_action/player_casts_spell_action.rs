use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An action for when a creature attacks.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerCastsSpellAction {
    pub player: crate::ability_tree::player::PassivePlayerReference,
    pub spell: crate::ability_tree::object::Spell,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for PlayerCastsSpellAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerCastsSpellAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        children.push(&self.spell as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player casts spell action:")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "spell:")?;
        out.push_final_branch()?;
        self.spell.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player casts spell action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerCastsSpellAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PlayerCastsSpellAction {
    fn default() -> Self {
        Self {
            player: Default::default(),
            spell: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
