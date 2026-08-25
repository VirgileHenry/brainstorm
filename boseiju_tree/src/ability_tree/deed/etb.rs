use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Permanent enters the battlfield deed.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntersTheBattlefield {
    /// The permanent that enters the battlfield.
    ///
    /// This can only ever be a passive reference.
    pub permanent: crate::ability_tree::object::PassivePermanent,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for EntersTheBattlefield {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::PayLife)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.permanent as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield:")?;
        out.push_final_branch()?;
        write!(out, "permanent:")?;
        out.push_final_branch()?;
        self.permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EntersTheBattlefield {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for EntersTheBattlefield {
    fn default() -> Self {
        Self {
            permanent: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
