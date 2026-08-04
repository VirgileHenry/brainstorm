use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::specified_object::SpecifiedCreature;
use crate::ability_tree::object::specified_object::SpecifiedPlaneswalker;
use crate::ability_tree::player::PlayerReference;

/// Any object that can receive damages.
///
/// For now, this is creatures, planewalkers, battles, players ?
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageReceiverKind {
    Creature(SpecifiedCreature),
    OneAmong(OneAmong<Self>),
    Planeswalker(SpecifiedPlaneswalker),
    Player(PlayerReference),
}

impl crate::Node for DamageReceiverKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::DamageReceiverKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Creature(child) => children.push(child as &dyn Node),
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::Planeswalker(child) => children.push(child as &dyn Node),
            Self::Player(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DamageReceiverKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Creature(child) => child.span(),
            Self::OneAmong(child) => child.span(),
            Self::Planeswalker(child) => child.span(),
            Self::Player(child) => child.span(),
        }
    }
}

impl Default for DamageReceiverKind {
    fn default() -> Self {
        Self::Creature(Default::default())
    }
}
