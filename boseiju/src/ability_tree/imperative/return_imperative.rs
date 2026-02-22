use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ReturnImperative {
    pub object: crate::ability_tree::object::ObjectReference,
    pub from: crate::ability_tree::zone::ZoneReference,
    pub to: crate::ability_tree::zone::ZoneReference,
}

impl AbilityTreeNode for ReturnImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ReturnImperative.id()
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
        write!(out, "return:")?;
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
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ReturnImperative {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            from: crate::utils::dummy(),
            to: crate::utils::dummy(),
        }
    }
}
