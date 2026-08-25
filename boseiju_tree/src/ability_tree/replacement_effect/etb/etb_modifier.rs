mod etb_perform_action;
mod etb_with_counters;
mod etb_with_state;

pub use etb_perform_action::EtbPerformAction;
pub use etb_with_counters::EtbWithCounters;
pub use etb_with_state::EtbWithState;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtbModifier {
    WithCounters(EtbWithCounters),
    WithState(EtbWithState),
    PerformAction(EtbPerformAction),
}

impl Node for EtbModifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EtbModifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::WithCounters(child) => children.push(child as &dyn Node),
            Self::WithState(child) => children.push(child as &dyn Node),
            Self::PerformAction(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield modifier:")?;
        out.push_final_branch()?;
        match self {
            Self::WithCounters(child) => child.display(out)?,
            Self::WithState(child) => child.display(out)?,
            Self::PerformAction(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counter on permanent replacement"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EtbModifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::WithCounters(child) => child.span(),
            Self::WithState(child) => child.span(),
            Self::PerformAction(child) => child.span(),
        }
    }
}

impl Default for EtbModifier {
    fn default() -> Self {
        Self::WithState(Default::default())
    }
}
