mod all;
mod count_quantifier;
mod target_quantifier;

pub use all::All;
pub use count_quantifier::CountQuantifier;
pub use target_quantifier::TargetQuantifier;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The quantifier node tells how much of something we are interested in.
///
/// Fixme: this include too much stuff:
/// Triggered abilities (whenever a creature...) should not be a active quantifier
/// like "destroy target/all creature" are. Sooo perhaps something to change here ?
///
/// If we want a clean tree, we can't leave it as is
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantifier {
    All(All),
    Count(CountQuantifier),
    Target(TargetQuantifier),
}

impl Node for Quantifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Quantifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::All(child) => children.push(child as &dyn Node),
            Self::Count(child) => children.push(child as &dyn Node),
            Self::Target(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "quantifier:")?;
        out.push_final_branch()?;
        match self {
            Self::All(child) => child.display(out)?,
            Self::Count(child) => child.display(out)?,
            Self::Target(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "quantifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Quantifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::All(child) => child.span(),
            Self::Count(child) => child.span(),
            Self::Target(child) => child.span(),
        }
    }
}

impl Default for Quantifier {
    fn default() -> Self {
        Self::All(Default::default())
    }
}
