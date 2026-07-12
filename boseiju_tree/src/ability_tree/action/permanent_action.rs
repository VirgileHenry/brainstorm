mod permanent_etb_action;

pub use permanent_etb_action::PermanentEtbAction;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// An action a permanent can perform.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentAction {
    EntersTheBattlefield(PermanentEtbAction),
}

impl crate::Node for PermanentAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PermanentAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::EntersTheBattlefield(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature action:")?;
        out.push_final_branch()?;
        match self {
            Self::EntersTheBattlefield(event) => event.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentAction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::EntersTheBattlefield(child) => child.span(),
        }
    }
}

impl Default for PermanentAction {
    fn default() -> Self {
        Self::EntersTheBattlefield(Default::default())
    }
}
