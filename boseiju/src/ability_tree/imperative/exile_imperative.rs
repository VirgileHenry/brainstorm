use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ExileImperative {
    pub object: crate::ability_tree::object::ObjectReference,
    pub follow_up: Option<ExileFollowUp>,
}

impl crate::ability_tree::AbilityTreeNode for ExileImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DestroyImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn AbilityTreeNode);
        match self.follow_up.as_ref() {
            Some(follow_up) => children.push(follow_up as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "exile:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        match self.follow_up.as_ref() {
            Some(follow_up) => {
                write!(out, "and then:")?;
                out.push_final_branch()?;
                follow_up.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "and then: that's it")?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// List of things that can happen after exiling stuff
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ExileFollowUp {
    ReturnIt(ExileFollowUpReturn),
}

impl AbilityTreeNode for ExileFollowUp {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExileFollowUp.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ReturnIt(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "exile follow up:")?;
        out.push_final_branch()?;
        match self {
            Self::ReturnIt(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// Follow up to return an object after exiling it.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ExileFollowUpReturn {
    pub return_imperative: crate::ability_tree::imperative::ReturnImperative,
    pub at: Option<crate::ability_tree::time::Instant>,
}

impl AbilityTreeNode for ExileFollowUpReturn {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExileFollowUpReturn.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.return_imperative as &dyn AbilityTreeNode);
        match self.at.as_ref() {
            Some(at) => children.push(at as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "return:")?;
        out.push_inter_branch()?;
        write!(out, "return imperative:")?;
        out.push_final_branch()?;
        self.return_imperative.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "at:")?;
        out.push_final_branch()?;
        match self.at.as_ref() {
            Some(at) => at.display(out)?,
            None => write!(out, "immediatly")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
