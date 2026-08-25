use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbWithCounters {
    pub counter_kind: boseiju_lexer::terminal::Counter,
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for EtbWithCounters {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EtbWithCounters
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.counter_kind as &dyn Node);
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield with counters:")?;
        out.push_inter_branch()?;
        write!(out, "counter kind:")?;
        out.push_final_branch()?;
        self.counter_kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EtbWithCounters {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for EtbWithCounters {
    fn default() -> Self {
        Self {
            counter_kind: Default::default(),
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
