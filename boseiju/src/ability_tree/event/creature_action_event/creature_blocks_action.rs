use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Action for a creature to deal combat damage.
///
/// Combat damage is the special kind of damage that creature deals when
/// they fight each other, or when they attack a player.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureBlocksAction {
    pub blocked_creature: Option<crate::ability_tree::object::ObjectReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for CreatureBlocksAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureBlocksAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        match self.blocked_creature.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature blocks")?;
        out.push_final_branch()?;
        write!(out, "blocked creature:")?;
        out.push_final_branch()?;
        match self.blocked_creature.as_ref() {
            Some(child) => child.display(out)?,
            None => write!(out, "any")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature action: blocks"
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
            blocked_creature: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
