mod counter_on_permanent;
mod enters_the_battlefield;
mod source_ref;
mod token_creation;

pub use counter_on_permanent::CounterOnPermanentReplacement;
pub use enters_the_battlefield::*;
pub use source_ref::EventSourceReference;
pub use token_creation::TokenCreationReplacement;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An event replacement kind.
///
/// A replacement effect replaces an event with another,
/// and this represent the event replacer.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventReplacement {
    CounterOnPermanent(CounterOnPermanentReplacement),
    EntersTheBattlefield(EtbReplacement),
    TokenCreation(TokenCreationReplacement),
}

#[cfg(feature = "spanned_tree")]
impl EventReplacement {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CounterOnPermanent(child) => child.span,
            Self::EntersTheBattlefield(child) => child.span,
            Self::TokenCreation(child) => child.span,
        }
    }
}

impl crate::ability_tree::AbilityTreeNode for EventReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EventReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CounterOnPermanent(child) => children.push(child as &dyn AbilityTreeNode),
            Self::EntersTheBattlefield(child) => children.push(child as &dyn AbilityTreeNode),
            Self::TokenCreation(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event replacement")?;
        out.push_final_branch()?;
        match self {
            Self::CounterOnPermanent(child) => child.display(out)?,
            Self::EntersTheBattlefield(child) => child.display(out)?,
            Self::TokenCreation(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "event replacement"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CounterOnPermanent(child) => child.node_span(),
            Self::EntersTheBattlefield(child) => child.node_span(),
            Self::TokenCreation(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EventReplacement {
    fn dummy_init() -> Self {
        Self::TokenCreation(crate::utils::dummy())
    }
}
