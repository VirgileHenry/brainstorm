use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An imperative for returning an object from one zone to another.
///
/// The imperative is called "return" as it's usually how it's phrased in cards,
/// but it mostly moves the object from a zone to another.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeZoneImperative {
    pub object: crate::ability_tree::object::CardReference,
    pub from: crate::ability_tree::zone::ZoneReference,
    pub to: crate::ability_tree::zone::ZoneReference,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ChangeZoneImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ChangeZoneImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        children.push(&self.from as &dyn AbilityTreeNode);
        children.push(&self.to as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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
        "return imperative"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ChangeZoneImperative {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            from: crate::utils::dummy(),
            to: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
