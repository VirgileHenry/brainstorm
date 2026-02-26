use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A zone that is owned by a player: libraries, hands, graveyards.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct OwnedZone {
    pub zone: crate::ability_tree::zone::OwnableZone,
    pub owner: crate::ability_tree::terminals::OwnerSpecifier,
}

impl AbilityTreeNode for OwnedZone {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::OwnedZone.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.zone as &dyn AbilityTreeNode);
        children.push(&self.owner as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "owned zone:")?;
        out.push_inter_branch()?;
        write!(out, "zone: ")?;
        self.zone.display(out)?;
        out.next_final_branch()?;
        write!(out, "owner: ")?;
        self.owner.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

impl idris::Idris for OwnedZone {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "owned zone"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnedZone {
    fn dummy_init() -> Self {
        Self {
            zone: crate::utils::dummy(),
            owner: crate::utils::dummy(),
        }
    }
}
