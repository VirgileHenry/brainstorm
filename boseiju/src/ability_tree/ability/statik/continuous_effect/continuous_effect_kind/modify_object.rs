mod object_characteristic_modification;

pub use object_characteristic_modification::*;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_OBJECT_MODIFICATIONS: usize = MAX_CHILDREN_PER_NODE - 1;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyObjectEffect {
    pub object: crate::ability_tree::object::PermanentReference,
    pub modifications: crate::utils::HeapArrayVec<ObjectAbilitiesModification, MAX_OBJECT_MODIFICATIONS>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ModifyObjectEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ModifyObjectEffect.id()
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
        write!(out, "modify object effect:")?;
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

    fn node_tag(&self) -> &'static str {
        "modify object effect"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ModifyObjectEffect {
    fn dummy_init() -> Self {
        Self {
            object: crate::utils::dummy(),
            modifications: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A modification to an object.
///
/// Fixme: some of those only works on creatures, they should be properly separated
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectAbilitiesModification {
    CharacteristicModification(ObjectCharacteristicModification), /* Fixme: some of these are only for creatures */
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

    fn node_tag(&self) -> &'static str {
        "object modification"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CharacteristicModification(child) => child.node_span(),
            Self::GainAbility(child) => child.node_span(),
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectGainAbility {
    pub ability: crate::ability_tree::ability::Ability,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "object gain ability modification"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ObjectGainAbility {
    fn dummy_init() -> Self {
        Self {
            ability: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
