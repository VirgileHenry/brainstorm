use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Statement {
    Imperative(crate::ability_tree::imperative::Imperative),
    May(MayAbility),
}

impl crate::ability_tree::AbilityTreeNode for Statement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Statement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::Imperative(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::May(ability) => abilities.push(ability as &dyn AbilityTreeNode),
        };
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Statement::Imperative(imp) => imp.display(out),
            Statement::May(may) => may.display(out),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Statement {
    fn dummy_init() -> Self {
        Self::Imperative(crate::utils::dummy())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct MayAbility {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub action: crate::ability_tree::imperative::Imperative,
    /* Fixme: If they don't / if they do */
}

impl crate::ability_tree::AbilityTreeNode for MayAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::MayAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.player as &dyn AbilityTreeNode);
        abilities.push(&self.action as &dyn AbilityTreeNode);
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "may ability")?;
        out.push_inter_branch()?;
        write!(out, "player: ")?;
        self.player.display(out)?;
        out.next_final_branch()?;
        write!(out, "may: ")?;
        self.action.display(out)?;
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for MayAbility {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            action: crate::utils::dummy(),
        }
    }
}
