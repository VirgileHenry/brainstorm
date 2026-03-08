use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event for when a permanent enters the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntersTheBattlefieldEvent {
    pub object: crate::ability_tree::object::ObjectReference,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for EntersTheBattlefieldEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EntersTheBattlefieldEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object enters the battlefield:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield event"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EntersTheBattlefieldEvent {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
