use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An event for when a creature performs a creature action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPerformsActionEvent {
    pub action: crate::ability_tree::action::PlayerAction,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for PlayerPerformsActionEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerPerformsActionEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.action as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player performs action event:")?;
        out.push_final_branch()?;
        write!(out, "action:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player performs action event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerPerformsActionEvent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PlayerPerformsActionEvent {
    fn default() -> Self {
        Self {
            action: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
