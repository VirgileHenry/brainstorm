use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A condition that is met when a given event has occured in a given timeframe.
///
/// Examples are, "if you attacked this turn" or "if a creature died this turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionEventOccured {
    pub event: crate::ability_tree::event::Event,
    pub timeframe: boseiju_lexer::terminal::BackwardDuration,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionEventOccured {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ConditionEventOccured
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.event as &dyn Node);
        children.push(&self.timeframe as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event occured:")?;
        out.push_inter_branch()?;
        self.event.display(out)?;
        out.next_final_branch()?;
        write!(out, "time frame:")?;
        out.push_final_branch()?;
        self.timeframe.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "event occured"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionEventOccured {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionEventOccured {
    fn default() -> Self {
        Self {
            event: Default::default(),
            timeframe: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
