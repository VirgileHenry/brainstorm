mod number_of_permanents;

pub use number_of_permanents::NumberOfPermanents;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A number derived from the current state of the game.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStateNumber {
    NumberOfPermanents(NumberOfPermanents),
}

impl Node for GameStateNumber {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::GameStateNumber
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NumberOfPermanents(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for GameStateNumber {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::NumberOfPermanents(child) => child.span(),
        }
    }
}

impl Default for GameStateNumber {
    fn default() -> Self {
        Self::NumberOfPermanents(Default::default())
    }
}
