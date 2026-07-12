use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An event for when a permanent gains a permanent state.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentGainsStateEvent {
    pub permanent: crate::ability_tree::object::Permanent,
    pub state: crate::ability_tree::state::PermanentState,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for PermanentGainsStateEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PermanentGainsStateEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn Node);
        children.push(&self.state as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent gains state event:")?;
        out.push_inter_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "gains state:")?;
        out.push_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent gains state event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentGainsStateEvent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PermanentGainsStateEvent {
    fn default() -> Self {
        Self {
            permanent: Default::default(),
            state: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
