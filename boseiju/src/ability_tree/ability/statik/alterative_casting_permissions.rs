use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// An alternative casting permissions grants the right to cast
/// a card from another zone than your hand.
///
/// For example, gravecrawler says: "You may cast this card from your graveyard".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct AlternativeCastingPermissions {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub object: crate::ability_tree::object::ObjectReference,
    pub from_zone: crate::ability_tree::zone::ZoneReference,
    pub additional_cost: Option<crate::ability_tree::cost::Cost>,
}

impl crate::ability_tree::AbilityTreeNode for AlternativeCastingPermissions {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ContinuousEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        children.push(&self.object as &dyn AbilityTreeNode);
        children.push(&self.from_zone as &dyn AbilityTreeNode);
        match self.additional_cost.as_ref() {
            Some(cost) => children.push(cost as &dyn AbilityTreeNode),
            None => {
                children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "alternative casting permission:")?;
        out.push_inter_branch()?;
        write!(out, "player specifier:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "objects:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "from zone:")?;
        out.push_final_branch()?;
        self.from_zone.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "additional cost:")?;
        out.push_final_branch()?;
        match self.additional_cost.as_ref() {
            Some(cost) => cost.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AlternativeCastingPermissions {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            object: crate::utils::dummy(),
            from_zone: crate::utils::dummy(),
            additional_cost: None,
        }
    }
}
