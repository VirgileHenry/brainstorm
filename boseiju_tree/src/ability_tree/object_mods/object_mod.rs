pub mod creature_power_toughness_modifier;
pub mod object_gain_ability;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A modification to an object.
///
/// Fixme: some of those only works on creatures, they should be properly separated
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectMod {
    /// Fixme: some of these are only for creatures
    CreaturePowerToughness(creature_power_toughness_modifier::CreaturePowerToughnessModifier),
    GainAbility(ObjectGainAbility),
}

impl Node for ObjectMod {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ObjectAbilitiesMod
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::CreaturePowerToughness(child) => children.push(child as &dyn Node),
            Self::GainAbility(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object modification:")?;
        out.push_final_branch()?;
        match self {
            Self::CreaturePowerToughness(child) => child.display(out)?,
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
impl boseiju_span::Spanned for ObjectMod {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::CreaturePowerToughness(child) => child.span(),
            Self::GainAbility(child) => child.span(),
        }
    }
}

impl Default for ObjectMod {
    fn default() -> Self {
        Self::CreaturePowerToughness(Default::default())
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
