use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::AnotherObjectSpecifier;
use crate::ability_tree::object::specified_object::ColorSpecifier;
use crate::ability_tree::object::specified_object::ControlSpecifier;
use crate::ability_tree::object::specified_object::OwnerSpecifier;
use crate::ability_tree::object::specified_object::Specifier;

/// Specifiers for permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentSpecifier {
    Another(AnotherObjectSpecifier),
    Color(ColorSpecifier),
    Control(ControlSpecifier),
    Owner(OwnerSpecifier),
}

impl PermanentSpecifier {
    pub fn to_artifact_specifier(&self) -> crate::ability_tree::object::specified_object::ArtifactSpecifier {
        use crate::ability_tree::object::specified_object::ArtifactSpecifier;
        match self {
            Self::Another(another) => ArtifactSpecifier::Another(another.clone()),
            Self::Color(color) => ArtifactSpecifier::Color(color.clone()),
            Self::Control(control) => ArtifactSpecifier::Control(control.clone()),
            Self::Owner(owner) => ArtifactSpecifier::Owner(owner.clone()),
        }
    }
    pub fn to_creature_specifier(&self) -> crate::ability_tree::object::specified_object::CreatureSpecifier {
        use crate::ability_tree::object::specified_object::CreatureSpecifier;
        match self {
            Self::Another(another) => CreatureSpecifier::Another(another.clone()),
            Self::Color(color) => CreatureSpecifier::Color(color.clone()),
            Self::Control(control) => CreatureSpecifier::Control(control.clone()),
            Self::Owner(owner) => CreatureSpecifier::Owner(owner.clone()),
        }
    }
    pub fn to_enchantment_specifier(&self) -> crate::ability_tree::object::specified_object::EnchantmentSpecifier {
        use crate::ability_tree::object::specified_object::EnchantmentSpecifier;
        match self {
            Self::Another(another) => EnchantmentSpecifier::Another(another.clone()),
            Self::Color(color) => EnchantmentSpecifier::Color(color.clone()),
            Self::Control(control) => EnchantmentSpecifier::Control(control.clone()),
            Self::Owner(owner) => EnchantmentSpecifier::Owner(owner.clone()),
        }
    }
    pub fn to_land_specifier(&self) -> crate::ability_tree::object::specified_object::LandSpecifier {
        use crate::ability_tree::object::specified_object::LandSpecifier;
        match self {
            Self::Another(another) => LandSpecifier::Another(another.clone()),
            Self::Color(color) => LandSpecifier::Color(color.clone()),
            Self::Control(control) => LandSpecifier::Control(control.clone()),
            Self::Owner(owner) => LandSpecifier::Owner(owner.clone()),
        }
    }
    pub fn to_planeswalker_specifier(&self) -> crate::ability_tree::object::specified_object::PlaneswalkerSpecifier {
        use crate::ability_tree::object::specified_object::PlaneswalkerSpecifier;
        match self {
            Self::Another(another) => PlaneswalkerSpecifier::Another(another.clone()),
            Self::Color(color) => PlaneswalkerSpecifier::Color(color.clone()),
            Self::Control(control) => PlaneswalkerSpecifier::Control(control.clone()),
            Self::Owner(owner) => PlaneswalkerSpecifier::Owner(owner.clone()),
        }
    }
}

impl Specifier for PermanentSpecifier {}

impl crate::ability_tree::AbilityTreeNode for PermanentSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Owner(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Another(child) => child.display(out)?,
            Self::Color(child) => child.display(out)?,
            Self::Control(child) => child.display(out)?,
            Self::Owner(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Another(child) => child.node_span(),
            Self::Color(child) => child.node_span(),
            Self::Control(child) => child.node_span(),
            Self::Owner(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentSpecifier {
    fn dummy_init() -> Self {
        Self::Control(crate::utils::dummy())
    }
}
