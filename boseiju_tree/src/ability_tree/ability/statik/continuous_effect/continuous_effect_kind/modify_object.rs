mod object_characteristic_modification;

pub use object_characteristic_modification::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_OBJECT_MODIFICATIONS: usize = MAX_CHILDREN_PER_NODE - 1;

/// A continuous effect that grants abilities to objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyObjectEffect {
    pub object: crate::ability_tree::object::Permanent,
    pub modifications: crate::HeapArrayVec<ObjectAbilitiesModification, MAX_OBJECT_MODIFICATIONS>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ModifyObjectEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ModifyObjectEffect
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        for modification in self.modifications.iter() {
            children.push(modification as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ModifyObjectEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ModifyObjectEffect {
    fn default() -> Self {
        Self {
            object: Default::default(),
            modifications: Default::default(),
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

impl Node for ObjectAbilitiesModification {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ObjectAbilitiesModification
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CharacteristicModification(child) => children.push(child as &dyn Node),
            Self::GainAbility(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectAbilitiesModification {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CharacteristicModification(child) => child.span(),
            Self::GainAbility(child) => child.span(),
        }
    }
}

impl Default for ObjectAbilitiesModification {
    fn default() -> Self {
        Self::CharacteristicModification(Default::default())
    }
}

/// An object modification that grants a new ability to that object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectGainAbility {
    pub ability: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ObjectGainAbility {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ObjectGainAbility
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.ability as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectGainAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ObjectGainAbility {
    fn default() -> Self {
        Self {
            ability: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
