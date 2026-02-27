use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum EventSource {
    AnEffect(EffectEventSource),
    Player(crate::ability_tree::terminals::PlayerSpecifier),
}

impl crate::ability_tree::AbilityTreeNode for EventSource {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EventSource.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            EventSource::AnEffect(child) => children.push(child as &dyn AbilityTreeNode),
            EventSource::Player(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::AnEffect(source) => source.display(out)?,
            Self::Player(player) => {
                write!(out, "event source: ")?;
                player.display(out)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EventSource {
    fn dummy_init() -> Self {
        Self::AnEffect(crate::utils::dummy())
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct EffectEventSource;

impl crate::ability_tree::AbilityTreeNode for EffectEventSource {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EffectEventSource.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "event source: an effect")
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EffectEventSource {
    fn dummy_init() -> Self {
        Self
    }
}
