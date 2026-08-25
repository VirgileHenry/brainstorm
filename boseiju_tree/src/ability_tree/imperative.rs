use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An imperative is an instruction a player must follow.
/// It represents something that shall be done.
///
/// At the structure level, an imperative is a deed with an active form.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imperative {
    pub action: PlayerAction,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for Imperative {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::Imperative
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.action as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative:")?;
        out.push_final_branch()?;
        write!(out, "action:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Imperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for Imperative {
    fn default() -> Self {
        Self {
            action: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// All actions players can take.
///
/// This is a sublist of the deeds in the active form, acting
/// as a barrier to prevent any deed from being a player action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    AddMana(crate::ability_tree::deed::add_mana::AddManaActive),
    DealDamages(crate::ability_tree::deed::deal_damages::DealDamages),
    Destroy(crate::ability_tree::deed::destroy::DestroyActive),
    Draw(crate::ability_tree::deed::draw::DrawActive),
    Modal(ModalAction),
    PutCounters(crate::ability_tree::deed::put_counters::PutCountersActive),
}

impl Node for PlayerAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerAction
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AddMana(child) => children.push(child as &dyn Node),
            Self::DealDamages(child) => children.push(child as &dyn Node),
            Self::Destroy(child) => children.push(child as &dyn Node),
            Self::Draw(child) => children.push(child as &dyn Node),
            Self::Modal(child) => children.push(child as &dyn Node),
            Self::PutCounters(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player action:")?;
        out.push_final_branch()?;
        match self {
            Self::AddMana(child) => child.display(out)?,
            Self::DealDamages(child) => child.display(out)?,
            Self::Destroy(child) => child.display(out)?,
            Self::Draw(child) => child.display(out)?,
            Self::Modal(child) => child.display(out)?,
            Self::PutCounters(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player action"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerAction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AddMana(child) => child.span(),
            Self::DealDamages(child) => child.span(),
            Self::Destroy(child) => child.span(),
            Self::Draw(child) => child.span(),
            Self::Modal(child) => child.span(),
            Self::PutCounters(child) => child.span(),
        }
    }
}

impl Default for PlayerAction {
    fn default() -> Self {
        Self::Destroy(Default::default())
    }
}

const MAX_CHOICES: usize = MAX_CHILDREN_PER_NODE - 1;

/// An imperative that requires a player to choose between different clauses.
///
/// This is common in modal effects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalAction {
    pub mode_count: crate::ability_tree::number::Number,
    pub can_choose_same_mode: bool,
    pub modes: crate::HeapArrayVec<crate::ability_tree::ability::spell::SpellAbility, MAX_CHOICES>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ModalAction {
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
impl boseiju_span::Spanned for ModalAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ModalAction {
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
