use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::kind::CreatureKind;
use crate::ability_tree::object::kind::EnchantmentKind;
use crate::ability_tree::object::kind::LandKind;
use crate::ability_tree::object::kind::PlaneswalkerKind;
use crate::ability_tree::object::kind::artifact_kind::ArtifactKind;
use crate::ability_tree::object::specified_object::PermanentSpecifier;
use crate::ability_tree::object::specified_object::SpecifiedPermanent;

/// An permanent reference is a way to refer specifically to permanent kinds.
/// This is useful when the action only makes sense for permanents.
///
/// For instance, an event that requires something to become tapped only makes sense
/// for permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentKind {
    Artifact(ArtifactKind),
    Creature(CreatureKind),
    Enchantment(EnchantmentKind),
    Land(LandKind),
    OneAmong(OneAmong<Self>),
    Planeswalker(PlaneswalkerKind),
    Specified(SpecifiedPermanent),
}

impl PermanentKind {
    pub fn add_factor_specifier(&self, factor_specifier: PermanentSpecifier) -> Self {
        match self {
            Self::Artifact(artifact) => {
                let artifact_specifier = factor_specifier.to_artifact_specifier();
                Self::Artifact(artifact.add_factor_specifier(artifact_specifier))
            }
            Self::Creature(artifact) => {
                let creature_specifier = factor_specifier.to_creature_specifier();
                Self::Creature(artifact.add_factor_specifier(creature_specifier))
            }
            Self::Enchantment(artifact) => {
                let enchantment_specifier = factor_specifier.to_enchantment_specifier();
                Self::Enchantment(artifact.add_factor_specifier(enchantment_specifier))
            }
            Self::Land(artifact) => {
                let land_specifier = factor_specifier.to_land_specifier();
                Self::Land(artifact.add_factor_specifier(land_specifier))
            }
            Self::OneAmong(one_among) => {
                let mut references = crate::utils::HeapArrayVec::new();
                for prev in one_among.references.iter() {
                    references.push(prev.add_factor_specifier(factor_specifier.clone()));
                }
                Self::OneAmong(OneAmong {
                    references,
                    #[cfg(feature = "spanned_tree")]
                    span: one_among.node_span().merge(&factor_specifier.node_span()),
                })
            }
            Self::Planeswalker(artifact) => {
                let planeswalker_specifier = factor_specifier.to_planeswalker_specifier();
                Self::Planeswalker(artifact.add_factor_specifier(planeswalker_specifier))
            }
            Self::Specified(specified) => Self::Specified(specified.add_factor_specifier(factor_specifier)),
        }
    }
}

impl crate::ability_tree::AbilityTreeNode for PermanentKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Artifact(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Creature(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Enchantment(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Land(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Planeswalker(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "permanent reference:")?;
        out.push_final_branch()?;
        match self {
            Self::Artifact(child) => child.display(out)?,
            Self::Creature(child) => child.display(out)?,
            Self::Enchantment(child) => child.display(out)?,
            Self::Land(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::Planeswalker(child) => child.display(out)?,
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "permanent reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Artifact(child) => child.node_span(),
            Self::Creature(child) => child.node_span(),
            Self::Enchantment(child) => child.node_span(),
            Self::Land(child) => child.node_span(),
            Self::OneAmong(child) => child.node_span(),
            Self::Planeswalker(child) => child.node_span(),
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentKind {
    fn dummy_init() -> Self {
        Self::Specified(crate::utils::dummy())
    }
}
