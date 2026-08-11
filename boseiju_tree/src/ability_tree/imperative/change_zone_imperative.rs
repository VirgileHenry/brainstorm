use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An imperative for returning an object from one zone to another.
///
/// The imperative is called "return" as it's usually how it's phrased in cards,
/// but it mostly moves the object from a zone to another.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeZoneImperative {
    pub object: crate::ability_tree::object::Card,
    pub from: crate::ability_tree::zone::ZoneReference,
    pub to: crate::ability_tree::zone::ZoneReference,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ChangeZoneImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ChangeZoneImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        children.push(&self.from as &dyn Node);
        children.push(&self.to as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "change zone:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "from:")?;
        out.push_final_branch()?;
        self.from.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "change zone imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ChangeZoneImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ChangeZoneImperative {
    fn default() -> Self {
        Self {
            object: Default::default(),
            from: Default::default(),
            to: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
