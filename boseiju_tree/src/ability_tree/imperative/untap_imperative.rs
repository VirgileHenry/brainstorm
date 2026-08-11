use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An imperative for untapping an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntapImperative {
    pub object: crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for UntapImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::UntapImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "untap:")?;
        out.push_final_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "untap imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for UntapImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for UntapImperative {
    fn default() -> Self {
        Self {
            object: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
