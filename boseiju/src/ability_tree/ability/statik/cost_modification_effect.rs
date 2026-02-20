use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Modification of the cost of objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationEffect {
    pub applies_to: crate::ability_tree::object::ObjectReference,
    pub modification: CostModification,
    pub condition: Option<crate::ability_tree::if_condition::IfCondition>,
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
        match self.condition.as_ref() {
            Some(condition) => children.push(condition as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
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

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostMore {
    more: crate::ability_tree::terminals::ManaCost,
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

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostLess {
    less: crate::ability_tree::terminals::ManaCost,
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

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationCostSet {
    set: crate::ability_tree::terminals::ManaCost,
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
