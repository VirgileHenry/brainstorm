use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Condition for a trigger ability.
///
/// This is always an event, and can optionnaly have conditions for the event to happen.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TriggerCondition {
    pub event: crate::ability_tree::event::Event,
    pub condition: Option<crate::ability_tree::conditional::Conditional>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for TriggerCondition {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TriggerCondition.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.event as &dyn AbilityTreeNode);
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn AbilityTreeNode),
            None => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "trigger condition:")?;
        out.push_inter_branch()?;
        self.event.display(out)?;
        out.next_final_branch()?;
        match self.condition.as_ref() {
            Some(condition) => condition.display(out)?,
            None => write!(out, "condition: none")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "trigger condition"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for TriggerCondition {
    fn dummy_init() -> Self {
        Self {
            event: crate::utils::dummy(),
            condition: None,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
