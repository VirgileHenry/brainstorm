use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A condition that is met when a given event has occured in a given timeframe.
///
/// Examples are, "if you attacked this turn" or "if a creature died this turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionEventOccured {
    pub event: crate::ability_tree::event::Event,
    pub timeframe: crate::ability_tree::time::BackwardDuration,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ConditionEventOccured {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ConditionEventOccured.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.event as &dyn AbilityTreeNode);
        children.push(&self.timeframe as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionEventOccured {
    fn dummy_init() -> Self {
        Self {
            event: crate::utils::dummy(),
            timeframe: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
