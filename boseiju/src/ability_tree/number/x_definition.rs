mod x_from_cost;
mod x_from_game_state;

pub use x_from_cost::XFromCost;
pub use x_from_game_state::XFromGameState;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XDefinition {
    FromCost(XFromCost),
    FromGameState(XFromGameState),
}

impl AbilityTreeNode for XDefinition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::XDefinition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        match self {
            Self::FromCost(child) => children.push(child as &dyn AbilityTreeNode),
            Self::FromGameState(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x definition")?;
        out.push_final_branch()?;
        match self {
            Self::FromCost(child) => child.display(out)?,
            Self::FromGameState(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x definition"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::FromCost(child) => child.node_span(),
            Self::FromGameState(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for XDefinition {
    fn dummy_init() -> Self {
        Self::FromGameState(crate::utils::dummy())
    }
}
