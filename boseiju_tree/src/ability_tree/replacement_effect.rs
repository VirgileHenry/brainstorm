pub mod etb;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A replacement effect replaces entirely one event with another.
///
/// See also <https://mtg.fandom.com/wiki/Replacement_effect>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplacementEffect {
    Etb(etb::EtbReplacementEffect),
}

impl Node for ReplacementEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ReplacementEffect
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Etb(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "replacement effect:")?;
        out.push_inter_branch()?;
        match self {
            Self::Etb(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "replacement event"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ReplacementEffect {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Etb(child) => child.span(),
        }
    }
}

impl Default for ReplacementEffect {
    fn default() -> Self {
        Self::Etb(Default::default())
    }
}
