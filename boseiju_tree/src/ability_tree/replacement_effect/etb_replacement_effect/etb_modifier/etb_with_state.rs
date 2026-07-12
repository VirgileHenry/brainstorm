use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbWithState {
    pub state: crate::ability_tree::state::PermanentState,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for EtbWithState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::EtbWithState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.state as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield with state:")?;
        out.push_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EtbWithState {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for EtbWithState {
    fn default() -> Self {
        Self {
            state: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
