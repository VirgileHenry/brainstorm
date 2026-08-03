use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An imperative for tapping an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XFromGameState {
    pub x_value: crate::ability_tree::number::GameStateNumber,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for XFromGameState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::XFromGameState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.x_value as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "x from game state:")?;
        out.push_final_branch()?;
        self.x_value.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "x from game state"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for XFromGameState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for XFromGameState {
    fn default() -> Self {
        Self {
            x_value: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
