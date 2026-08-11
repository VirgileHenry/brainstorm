use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/* Fixme: maybe "target object becomes tap" is better ?  */
/* We need a way for the AI to tell "tapping" is equivalent to "being tapped" */

/// An imperative for tapping an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapImperative {
    pub object: crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for TapImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::TapImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "tap:")?;
        out.push_final_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "tap imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TapImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for TapImperative {
    fn default() -> Self {
        Self {
            object: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
