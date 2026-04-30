use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::specified_object::SpecifiedArtifact;
use crate::ability_tree::object::specified_object::SpecifiedCreature;
use crate::ability_tree::object::specified_object::SpecifiedEnchantment;
use crate::ability_tree::object::specified_object::SpecifiedLand;
use crate::ability_tree::object::specified_object::SpecifiedPlaneswalker;

/// An permanent reference is a way to refer specifically to permanent kinds.
/// This is useful when the action only makes sense for permanents.
///
/// For instance, an event that requires something to become tapped only makes sense
/// for permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermanentKind {
    Artifact(SpecifiedArtifact),
    Creature(SpecifiedCreature),
    Enchantment(SpecifiedEnchantment),
    Land(SpecifiedLand),
    OneAmong(OneAmong<Self>),
    Planeswalker(SpecifiedPlaneswalker),
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl crate::ability_tree::AbilityTreeNode for PermanentKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PermanentKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Artifact(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Creature(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Enchantment(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Land(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Planeswalker(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Permanent { .. } => {
                let node_id = crate::ability_tree::NodeKind::PermanentBasicKind.id();
                let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn AbilityTreeNode)
            }
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
            Self::Permanent { .. } => write!(out, "permanent")?,
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
            Self::Permanent { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PermanentKind {
    fn dummy_init() -> Self {
        Self::Permanent {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
