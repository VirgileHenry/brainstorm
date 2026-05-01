use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::kind::CreatureKind;
use crate::ability_tree::object::kind::PlaneswalkerKind;
use crate::ability_tree::player::PlayerSpecifier;

/// Any object that can receive damages.
///
/// For now, this is creatures, planewalkers, battles, players ?
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageReceiverKind {
    Creature(CreatureKind),
    OneAmong(OneAmong<Self>),
    Planeswalker(PlaneswalkerKind),
    Player(PlayerSpecifier),
}

impl crate::ability_tree::AbilityTreeNode for DamageReceiverKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DamageReceiverKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Creature(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Planeswalker(child) => children.push(child as &dyn AbilityTreeNode),
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
            Self::OneAmong(child) => child.display(out)?,
            Self::Planeswalker(child) => child.display(out)?,
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
            Self::OneAmong(child) => child.node_span(),
            Self::Planeswalker(child) => child.node_span(),
            Self::Player(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for DamageReceiverKind {
    fn dummy_init() -> Self {
        Self::Creature(crate::utils::dummy())
    }
}
