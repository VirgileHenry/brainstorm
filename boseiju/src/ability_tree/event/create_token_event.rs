use super::*;
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CreateTokensEvent {
    pub source: source::EventSource,
    pub quantity: crate::ability_tree::number::Number,
    pub token_specifiers: Option<crate::ability_tree::object::ObjectSpecifiers>,
}

impl AbilityTreeNode for CreateTokensEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreateTokensEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source as &dyn AbilityTreeNode);
        children.push(&self.quantity as &dyn AbilityTreeNode);
        match self.token_specifiers.as_ref() {
            Some(specifiers) => children.push(specifiers as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "create tokens")?;
        out.push_inter_branch()?;
        write!(out, "token creation source:")?;
        out.push_final_branch()?;
        self.source.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.quantity.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        match self.token_specifiers.as_ref() {
            Some(specifiers) => {
                write!(out, "specifiers: ")?;
                specifiers.display(out)?;
            }
            None => write!(out, "specifiers: none")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreateTokensEvent {
    fn dummy_init() -> Self {
        Self {
            source: crate::utils::dummy(),
            quantity: crate::utils::dummy(),
            token_specifiers: crate::utils::dummy(),
        }
    }
}
