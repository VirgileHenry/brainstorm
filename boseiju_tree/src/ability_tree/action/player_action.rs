mod player_attacks_action;
mod player_casts_spell_action;

pub use player_attacks_action::PlayerAttacksAction;
pub use player_casts_spell_action::PlayerCastsSpellAction;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An action a player can perform.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Attacks(PlayerAttacksAction),
    CastsSpell(PlayerCastsSpellAction),
}

impl crate::Node for PlayerAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn Node),
            Self::CastsSpell(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player action:")?;
        out.push_final_branch()?;
        match self {
            Self::Attacks(event) => event.display(out)?,
            Self::CastsSpell(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerAction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Attacks(child) => child.span(),
            Self::CastsSpell(child) => child.span(),
        }
    }
}

impl Default for PlayerAction {
    fn default() -> Self {
        Self::Attacks(Default::default())
    }
}
