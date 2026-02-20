use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

const MAX_CHOICES: usize = MAX_CHILDREN_PER_NODE - 1;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ChooseImperative {
    pub choice_count: crate::ability_tree::number::Number,
    pub can_choose_same_mode: bool,
    pub choices: Box<arrayvec::ArrayVec<crate::ability_tree::ability::spell::SpellAbility, MAX_CHOICES>>,
}

impl AbilityTreeNode for ChooseImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ChooseImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.choice_count as &dyn AbilityTreeNode);
        for choice in self.choices.iter() {
            children.push(choice as &dyn AbilityTreeNode);
        }
        children
    }

    fn data(&self) -> arrayvec::ArrayVec<u8, MAX_NODE_DATA_SIZE> {
        /* Fixme: terrible for the ai */
        let mut data = arrayvec::ArrayVec::new_const();
        data.push(if self.can_choose_same_mode { 1 } else { 0 });
        data
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "choose:")?;
        out.push_inter_branch()?;
        write!(out, "number of choices:")?;
        out.push_final_branch()?;
        self.choice_count.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "can choose the same mode multiple times: {}", self.can_choose_same_mode)?;
        out.next_final_branch()?;
        write!(out, "choices:")?;
        for choice in self.choices.iter().take(self.choices.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        if let Some(choice) = self.choices.last() {
            out.push_final_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}
