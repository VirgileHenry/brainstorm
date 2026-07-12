use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// A DamageReceiver reference.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageReceiverReference {
    pub count: crate::ability_tree::object::CountSpecifier,
    pub kind: crate::ability_tree::object::kind::DamageReceiverKind, /* Fixme: specified */
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for DamageReceiverReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::DamageReceiverReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn Node);
        children.push(&self.kind as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "damage receiver reference:")?;
        out.push_inter_branch()?;
        write!(out, "count:")?;
        out.push_final_branch()?;
        self.count.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "damage receiver reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DamageReceiverReference {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for DamageReceiverReference {
    fn default() -> Self {
        Self {
            count: Default::default(),
            kind: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
