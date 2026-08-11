use crate::AbilityTreeNode;
use crate::MAX_CHILDREN_PER_NODE;
use crate::tree::object::specified_object::SpellSpecifier;

/// An event for when a player casts a spell.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerCastsSpellEvent {
    pub spell_specifiers: Option<SpellSpecifier>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::AbilityTreeNode for PlayerCastsSpellEvent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerCastsSpellEvent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self.spell_specifiers.as_ref() {
            Some(amount) => children.push(amount as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PlayerCastsSpellEvent {
    fn default() -> Self {
        Self {
            spell_specifiers: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
