use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Modification of the cost of objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationEffect {
    pub applies_to: crate::ability_tree::object::ObjectReference,
    pub modification: CostModification,
}

impl crate::ability_tree::AbilityTreeNode for CostModificationEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CostModificationEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.applies_to as &dyn AbilityTreeNode);
        children.push(&self.modification as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost modification effect:")?;
        out.push_inter_branch()?;
        write!(out, "applies to:")?;
        out.push_final_branch()?;
        self.applies_to.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        self.modification.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CostModificationEffect {
    fn dummy_init() -> Self {
        Self {
            applies_to: crate::utils::dummy(),
            modification: crate::utils::dummy(),
        }
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CostModification {
    More(CostModificationCostMore),
    Less(CostModificationCostLess),
    Set(CostModificationCostSet),
}

impl crate::ability_tree::AbilityTreeNode for CostModification {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CostModification.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::More(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Less(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Set(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost modification effect:")?;
        out.push_final_branch()?;
        match self {
            Self::More(child) => child.display(out)?,
            Self::Less(child) => child.display(out)?,
            Self::Set(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CostModification {
    fn dummy_init() -> Self {
        Self::Less(crate::utils::dummy())
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostMore {
    pub more: crate::ability_tree::terminals::ManaCost,
}

impl AbilityTreeNode for CostModificationCostMore {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CostModificationCostMore.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.more as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost {} more to cast", self.more)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CostModificationCostMore {
    fn dummy_init() -> Self {
        Self {
            more: crate::utils::dummy(),
        }
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostLess {
    pub less: crate::ability_tree::terminals::ManaCost,
}

impl AbilityTreeNode for CostModificationCostLess {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CostModificationCostLess.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.less as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost {} less to cast", self.less)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CostModificationCostLess {
    fn dummy_init() -> Self {
        Self {
            less: crate::utils::dummy(),
        }
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostSet {
    pub set: crate::ability_tree::terminals::ManaCost,
}

impl AbilityTreeNode for CostModificationCostSet {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CostModificationCostSet.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.set as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost {} to cast", self.set)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CostModificationCostSet {
    fn dummy_init() -> Self {
        Self {
            set: crate::utils::dummy(),
        }
    }
}
