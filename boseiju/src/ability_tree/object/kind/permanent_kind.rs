use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::kind::CreatureKind;
use crate::ability_tree::object::kind::LandKind;
use crate::ability_tree::object::kind::PlaneswalkerKind;
use crate::ability_tree::object::kind::artifact_kind::ArtifactKind;
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
    Land(LandKind),
    OneAmong(OneAmong<Self>),
    Creature(CreatureKind),
    Planeswalker(PlaneswalkerKind),
    Specified(SpecifiedPermanent),
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
