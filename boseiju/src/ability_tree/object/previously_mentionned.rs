use super::*;
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Anything from "that permanent", "those counters", "that card", etc.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PreviouslyMentionnedObject {
    pub kind: Option<ObjectKind>,
}

impl AbilityTreeNode for PreviouslyMentionnedObject {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PreviouslyMentionnedObject.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new();
        match self.kind.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "previously mentionned object:")?;
        out.push_final_branch()?;
        write!(out, "object kind:")?;
        match self.kind.as_ref() {
            Some(kind) => kind.display(out)?,
            None => write!(out, "any")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PreviouslyMentionnedObject {
    fn dummy_init() -> Self {
        Self {
            kind: crate::utils::dummy(),
        }
    }
}
