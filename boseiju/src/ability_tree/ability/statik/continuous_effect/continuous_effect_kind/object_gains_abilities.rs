use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ContinuousEffectObjectGainsAbilies {
    pub object: crate::ability_tree::object::ObjectReference,
    pub abilities: Box<crate::AbilityTree>,
}

impl AbilityTreeNode for ContinuousEffectObjectGainsAbilies {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectObjectGainsAbilies.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        children.push(self.abilities.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object gains ability:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "abilities:")?;
        out.push_final_branch()?;
        self.abilities.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffectObjectGainsAbilies {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            abilities: crate::utils::dummy(),
        }
    }
}
