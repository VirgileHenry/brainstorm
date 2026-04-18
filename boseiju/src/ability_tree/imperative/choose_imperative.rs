use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::MAX_NODE_DATA_SIZE;

const MAX_CHOICES: usize = MAX_CHILDREN_PER_NODE - 1;

/// An imperative that requires a player to choose between different clauses.
///
/// This is common in modal effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalImperative {
    pub mode_count: crate::ability_tree::number::Number,
    pub can_choose_same_mode: bool,
    pub modes: crate::utils::HeapArrayVec<crate::ability_tree::ability::spell::SpellAbility, MAX_CHOICES>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for ModalImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ChooseImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.mode_count as &dyn AbilityTreeNode);
        for choice in self.modes.iter() {
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
        write!(out, "modal:")?;
        out.push_inter_branch()?;
        write!(out, "number of choices:")?;
        out.push_final_branch()?;
        self.mode_count.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "can choose the same mode multiple times: {}", self.can_choose_same_mode)?;
        out.next_final_branch()?;
        write!(out, "choices:")?;
        for choice in self.modes.iter().take(self.modes.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        if let Some(choice) = self.modes.last() {
            out.push_final_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "modal imperative"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ModalImperative {
    fn dummy_init() -> Self {
        Self {
            mode_count: crate::utils::dummy(),
            can_choose_same_mode: false,
            modes: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
