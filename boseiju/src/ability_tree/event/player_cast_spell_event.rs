use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
/// Fixme: move into player actions ???
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlayerCastsSpellEvent {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub spell_specifiers: Option<crate::ability_tree::object::ObjectSpecifiers>,
}

impl crate::ability_tree::AbilityTreeNode for PlayerCastsSpellEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerCastsSpellEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        match self.spell_specifiers.as_ref() {
            Some(amount) => children.push(amount as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player casts a spell:")?;
        out.push_inter_branch()?;
        write!(out, "player: {}", self.player)?;
        out.next_final_branch()?;
        match self.spell_specifiers.as_ref() {
            Some(specifiers) => {
                write!(out, "spell specifiers:")?;
                out.push_final_branch()?;
                specifiers.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "spell specifiers: none")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerCastsSpellEvent {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            spell_specifiers: crate::utils::dummy(),
        }
    }
}
