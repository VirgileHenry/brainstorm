use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::CreatureReference;
use crate::ability_tree::player::PlayerSpecifier;

/// Any object that can receive damages.
///
/// For now, this is creatures, planewalkers, battles, players ?
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageReceiverReference {
    Creature(CreatureReference),
    Player(PlayerSpecifier),
}

impl crate::ability_tree::AbilityTreeNode for DamageReceiverReference {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DamageReceiverReference.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Creature(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Player(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "damage receiver reference:")?;
        out.push_final_branch()?;
        match self {
            Self::Creature(child) => child.display(out)?,
            Self::Player(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "damage receiver reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Creature(child) => child.node_span(),
            Self::Player(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for DamageReceiverReference {
    fn dummy_init() -> Self {
        Self::Creature(crate::utils::dummy())
    }
}
