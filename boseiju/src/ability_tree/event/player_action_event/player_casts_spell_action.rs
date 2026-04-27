use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::SpellSpecifier;

/// An event for when a player casts a spell.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerCastsSpellEvent {
    pub spell_specifiers: Option<SpellSpecifier>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PlayerCastsSpellEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerCastsSpellEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
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

    fn node_tag(&self) -> &'static str {
        "player action: cast spell"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerCastsSpellEvent {
    fn dummy_init() -> Self {
        Self {
            spell_specifiers: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
