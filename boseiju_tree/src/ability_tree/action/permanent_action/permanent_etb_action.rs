use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An action for when a permanent enters the battlefield.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentEtbAction {
    pub permanent: crate::ability_tree::object::PassivePermanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for PermanentEtbAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PermanentEtbAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent enters the battlefield action:")?;
        out.push_final_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent enters the battlefield action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentEtbAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PermanentEtbAction {
    fn default() -> Self {
        Self {
            permanent: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
