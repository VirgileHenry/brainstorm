// mod etb_perform_action;
mod etb_with_counters;
mod etb_with_state;

// pub use etb_perform_action::EtbPerformAction;
pub use etb_with_counters::EtbWithCounters;
pub use etb_with_state::EtbWithState;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtbModifier {
    WithCounters(EtbWithCounters),
    WithState(EtbWithState),
}

impl AbilityTreeNode for EtbModifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbModifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::WithCounters(child) => children.push(child as &dyn AbilityTreeNode),
            Self::WithState(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield modifier:")?;
        out.push_final_branch()?;
        match self {
            Self::WithCounters(child) => child.display(out)?,
            Self::WithState(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counter on permanent replacement"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::WithCounters(child) => child.node_span(),
            Self::WithState(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbModifier {
    fn dummy_init() -> Self {
        Self::WithState(crate::utils::dummy())
    }
}
