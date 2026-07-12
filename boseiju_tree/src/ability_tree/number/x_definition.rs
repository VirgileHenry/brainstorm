mod x_from_cost;
mod x_from_game_state;

pub use x_from_cost::XFromCost;
pub use x_from_game_state::XFromGameState;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XDefinition {
    FromCost(XFromCost),
    FromGameState(XFromGameState),
}

impl Node for XDefinition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::XDefinition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new();
        match self {
            Self::FromCost(child) => children.push(child as &dyn Node),
            Self::FromGameState(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for XDefinition {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::FromCost(child) => child.span(),
            Self::FromGameState(child) => child.span(),
        }
    }
}


impl Default for XDefinition {
    fn default() -> Self {
        Self::FromGameState(Default::default())
    }
}
