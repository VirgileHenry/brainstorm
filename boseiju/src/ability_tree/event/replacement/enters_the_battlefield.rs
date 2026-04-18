use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A replacement of when a permanent enters the battlefield.
///
/// From the CR:
/// \[614.1d\] Continuous effects that read "\[This permanent\] enters ..." \[...\] are replacement effects.
///
/// \[614.12\] Some replacement effects modify how a permanent enters the battlefield. \[...\] Such
/// effects may come from the permanent itself if they affect only that permanent\[.\]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbReplacement {
    pub source_ref: super::EventSourceReference,
    pub etb_modifiers: arrayvec::ArrayVec<EtbModifier, 23>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for EtbReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source_ref as &dyn AbilityTreeNode);
        for etb_modifier in self.etb_modifiers.iter() {
            children.push(etb_modifier as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield replacement:")?;
        out.push_inter_branch()?;
        write!(out, "effect source:")?;
        out.push_final_branch()?;
        self.source_ref.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "enters the battlefield modifiers:")?;
        for (i, etb_modifier) in self.etb_modifiers.iter().enumerate() {
            if i == self.etb_modifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            etb_modifier.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counter on permanent replacement"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbReplacement {
    fn dummy_init() -> Self {
        Self {
            source_ref: crate::utils::dummy(),
            etb_modifiers: arrayvec::ArrayVec::new_const(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtbModifier {
    PerformAction(EtbPerformAction),
    WithCounters(EtbWithCounters),
    WithState(EtbWithState),
}

impl AbilityTreeNode for EtbModifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbModifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::PerformAction(child) => children.push(child as &dyn AbilityTreeNode),
            Self::WithCounters(child) => children.push(child as &dyn AbilityTreeNode),
            Self::WithState(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield modifier:")?;
        out.push_final_branch()?;
        match self {
            Self::PerformAction(child) => child.display(out)?,
            Self::WithCounters(child) => child.display(out)?,
            Self::WithState(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counter on permanent replacement"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::PerformAction(child) => child.node_span(),
            Self::WithCounters(child) => child.node_span(),
            Self::WithState(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbModifier {
    fn dummy_init() -> Self {
        Self::WithState(crate::utils::dummy())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbWithCounters {
    pub counter_kind: crate::ability_tree::terminals::Counter,
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for EtbWithCounters {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbWithCounters.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.counter_kind as &dyn AbilityTreeNode);
        children.push(&self.amount as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield with counters:")?;
        out.push_inter_branch()?;
        write!(out, "counter kind:")?;
        out.push_final_branch()?;
        self.counter_kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbWithCounters {
    fn dummy_init() -> Self {
        Self {
            counter_kind: crate::utils::dummy(),
            amount: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbWithState {
    pub state: crate::ability_tree::terminals::CardState,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for EtbWithState {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbWithState.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.state as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield with state:")?;
        out.push_final_branch()?;
        self.state.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbWithState {
    fn dummy_init() -> Self {
        Self {
            state: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbPerformAction {
    pub action: crate::ability_tree::ability::spell::SpellAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for EtbPerformAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EtbPerformAction.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.action as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "perform action on etb:")?;
        out.push_final_branch()?;
        self.action.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield with counters"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EtbPerformAction {
    fn dummy_init() -> Self {
        Self {
            action: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
