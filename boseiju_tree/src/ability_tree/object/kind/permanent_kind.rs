use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
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
        span: boseiju_span::Span,
    },
}

impl crate::Node for PermanentKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PermanentKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Artifact(child) => children.push(child as &dyn Node),
            Self::Creature(child) => children.push(child as &dyn Node),
            Self::Enchantment(child) => children.push(child as &dyn Node),
            Self::Land(child) => children.push(child as &dyn Node),
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::Planeswalker(child) => children.push(child as &dyn Node),
            Self::Permanent { .. } => {
                let node_id = crate::NodeKind::PermanentBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PermanentKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Artifact(child) => child.span(),
            Self::Creature(child) => child.span(),
            Self::Enchantment(child) => child.span(),
            Self::Land(child) => child.span(),
            Self::OneAmong(child) => child.span(),
            Self::Planeswalker(child) => child.span(),
            Self::Permanent { span } => *span,
        }
    }
}

impl Default for PermanentKind {
    fn default() -> Self {
        Self::Permanent {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
