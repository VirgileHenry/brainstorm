use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Imperative to remove counters on permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberOfPermanents {
    pub permanent: crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for NumberOfPermanents {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::NumberOfPermanents.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "number of permanents:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "number of permanents"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NumberOfPermanents {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for NumberOfPermanents {
    fn default() -> Self {
        Self {
            permanent: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
