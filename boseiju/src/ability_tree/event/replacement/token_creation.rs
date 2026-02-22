use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_CREATED_TOKEN: usize = MAX_CHILDREN_PER_NODE - 1;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenCreationReplacement {
    pub source_ref: super::source_ref::EventSourceReference,
    pub create_tokens: crate::ability_tree::imperative::CreateTokenImperative,
}

impl AbilityTreeNode for TokenCreationReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TokenCreationReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source_ref as &dyn AbilityTreeNode);
        children.push(&self.create_tokens as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "token creation replacement:")?;
        out.push_inter_branch()?;
        write!(out, "effect source:")?;
        out.push_final_branch()?;
        self.source_ref.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "token creation:")?;
        out.next_final_branch()?;
        self.create_tokens.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for TokenCreationReplacement {
    fn dummy_init() -> Self {
        Self {
            source_ref: crate::utils::dummy(),
            create_tokens: crate::utils::dummy(),
        }
    }
}
