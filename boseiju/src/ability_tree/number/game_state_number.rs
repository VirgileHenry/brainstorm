mod number_of_permanents;

pub use number_of_permanents::NumberOfPermanents;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A number derived from the current state of the game.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStateNumber {
    NumberOfPermanents(NumberOfPermanents),
}

impl AbilityTreeNode for GameStateNumber {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::GameStateNumber.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NumberOfPermanents(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "game state number")?;
        out.push_final_branch()?;
        match self {
            Self::NumberOfPermanents(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "number"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::NumberOfPermanents(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for GameStateNumber {
    fn dummy_init() -> Self {
        Self::NumberOfPermanents(crate::utils::dummy())
    }
}
