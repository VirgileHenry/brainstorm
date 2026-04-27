use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbWithCounters {
    pub counter_kind: crate::ability_tree::terminals::Counter,
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for EtbWithCounters {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbWithCounters.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.counter_kind as &dyn AbilityTreeNode);
        children.push(&self.amount as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbWithCounters {
    fn dummy_init() -> Self {
        Self {
            counter_kind: crate::utils::dummy(),
            amount: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
