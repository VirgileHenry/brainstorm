mod player_attacks_action;
mod player_casts_spell_action;

pub use player_attacks_action::PlayerAttacksAction;
pub use player_casts_spell_action::PlayerCastsSpellAction;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An action a creature can perform.
///
/// These are the actions ONLY creatures can perform, there might be more actions
/// that can be performed by all permanents under the [`PermanentAction`] node.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Attacks(PlayerAttacksAction),
    CastsSpell(PlayerCastsSpellAction),
}

impl crate::ability_tree::AbilityTreeNode for PlayerAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attacks(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CastsSpell(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attacks(child) => child.node_span(),
            Self::CastsSpell(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerAction {
    fn dummy_init() -> Self {
        Self::Attacks(crate::utils::dummy())
    }
}
