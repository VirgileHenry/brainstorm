mod all;
mod count_quantifier;
mod target_quantifier;

pub use all::All;
pub use count_quantifier::CountQuantifier;
pub use target_quantifier::TargetQuantifier;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The quantifier trait allows to be generic over which quantifier is used.
///
/// The only quantifiers are active and passive.
pub trait Quantifier: Node + std::fmt::Debug + Clone + Eq + PartialEq {}

/// The active quantifier is used to quantify objects we want to act uppon.
///
/// To quantify them, it is by either targetting them or selecting them all.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveQuantifier {
    All(All),
    Target(TargetQuantifier),
}

impl Quantifier for ActiveQuantifier {}

impl Node for ActiveQuantifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ActiveQuantifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::All(child) => children.push(child as &dyn Node),
            Self::Target(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "active quantifier:")?;
        out.push_final_branch()?;
        match self {
            Self::All(child) => child.display(out)?,
            Self::Target(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "active quantifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ActiveQuantifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::All(child) => child.span(),
            Self::Target(child) => child.span(),
        }
    }
}

impl Default for ActiveQuantifier {
    fn default() -> Self {
        Self::All(Default::default())
    }
}

/// The passive quantifier is used to refer to objects to listen for events.
///
/// To quantify them, it is by either targetting them or selecting them all.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassiveQuantifier {
    Count(CountQuantifier),
}

impl Quantifier for PassiveQuantifier {}

impl Node for PassiveQuantifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PassiveQuantifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Count(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "active quantifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Count(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "active quantifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PassiveQuantifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Count(child) => child.span(),
        }
    }
}

impl Default for PassiveQuantifier {
    fn default() -> Self {
        Self::Count(Default::default())
    }
}
