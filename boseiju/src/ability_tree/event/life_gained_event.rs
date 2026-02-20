use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct LifeGainedEvent {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub minimum_amount: Option<crate::ability_tree::number::Number>,
}

impl crate::ability_tree::AbilityTreeNode for LifeGainedEvent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::LifeGainedEvent.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        match self.minimum_amount.as_ref() {
            Some(amount) => children.push(amount as &dyn AbilityTreeNode),
            None => {
                let dummy = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player gains life")?;
        out.push_inter_branch()?;
        write!(out, "player: {}", self.player)?;
        out.next_final_branch()?;
        match self.minimum_amount.as_ref() {
            Some(minimum_amount) => {
                write!(out, "gains at least:")?;
                out.push_final_branch()?;
                minimum_amount.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "gains any amount")?,
        }
        out.pop_branch();

        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for LifeGainedEvent {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            minimum_amount: crate::utils::dummy(),
        }
    }
}
