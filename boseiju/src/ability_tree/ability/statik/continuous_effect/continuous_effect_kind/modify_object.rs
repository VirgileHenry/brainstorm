mod object_characteristic_modification;

pub use object_characteristic_modification::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_OBJECT_MODIFICATIONS: usize = MAX_CHILDREN_PER_NODE - 1;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ContinuousEffectModifyObject {
    pub object: crate::ability_tree::object::ObjectReference,
    pub modifications: crate::utils::HeapArrayVec<ObjectAbilitiesModification, MAX_OBJECT_MODIFICATIONS>,
}

impl AbilityTreeNode for ContinuousEffectModifyObject {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffectModifyObject.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        for modification in self.modifications.iter() {
            children.push(modification as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object is modified:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "modifications:")?;
        for (i, modification) in self.modifications.iter().enumerate() {
            if i == self.modifications.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            modification.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ContinuousEffectModifyObject {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            modifications: crate::utils::dummy(),
        }
    }
}

/// A modification to an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectAbilitiesModification {
    CharacteristicModification(ObjectCharacteristicModification),
    GainAbility(ObjectGainAbility),
}

impl AbilityTreeNode for ObjectAbilitiesModification {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectAbilitiesModification.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CharacteristicModification(child) => children.push(child as &dyn AbilityTreeNode),
            Self::GainAbility(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object modification:")?;
        out.push_final_branch()?;
        match self {
            Self::CharacteristicModification(child) => child.display(out)?,
            Self::GainAbility(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectAbilitiesModification {
    fn dummy_init() -> Self {
        Self::CharacteristicModification(crate::utils::dummy())
    }
}

/// An object modification that grants a new ability to that object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ObjectGainAbility {
    pub ability: crate::ability_tree::ability::AbilityKind,
}

impl AbilityTreeNode for ObjectGainAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectGainAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.ability as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object gains ability:")?;
        out.push_final_branch()?;
        self.ability.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectGainAbility {
    fn dummy_init() -> Self {
        Self {
            ability: crate::utils::dummy(),
        }
    }
}
