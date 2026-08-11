use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A object reference that refers to one of multiple possible references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneAmong<T: Node> {
    pub references: crate::HeapArrayVec<T, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<T: Node> Node for OneAmong<T> {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::OneAmong
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.references.iter() {
            children.push(specifier as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "one among:")?;
        for (i, specifier) in self.references.iter().enumerate() {
            if i == self.references.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "one among reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl<T: Node> boseiju_span::Spanned for OneAmong<T> {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<T: Node> Default for OneAmong<T> {
    fn default() -> Self {
        Self {
            references: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
