use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_CHOICES: usize = MAX_CHILDREN_PER_NODE - 1;

/// An imperative that requires a player to choose between different clauses.
///
/// This is common in modal effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalImperative {
    pub mode_count: crate::ability_tree::number::Number,
    pub can_choose_same_mode: bool,
    pub modes: crate::HeapArrayVec<crate::ability_tree::ability::spell::SpellAbility, MAX_CHOICES>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ModalImperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ChooseImperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.mode_count as &dyn Node);
        for choice in self.modes.iter() {
            children.push(choice as &dyn Node);
        }
        children
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Boolean {
            value: self.can_choose_same_mode,
        })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ModalImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ModalImperative {
    fn default() -> Self {
        Self {
            mode_count: Default::default(),
            can_choose_same_mode: false,
            modes: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
