mod atomic_cost;

pub use atomic_cost::AtomicCost;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A cost is something that need to be paid.
///
/// It may be a mana cost (paying mana), or any imperative that requires
/// the player to do something (discard a card, sacrifice a creature...)
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    pub costs: crate::HeapArrayVec<atomic_cost::AtomicCost, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for Cost {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Cost
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.costs.iter() {
            children.push(child as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "costs:")?;
        for (i, child) in self.costs.iter().enumerate() {
            if i == self.costs.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            child.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Cost {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            costs: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
