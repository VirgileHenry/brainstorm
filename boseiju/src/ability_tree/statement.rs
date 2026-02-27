use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Statement {
    Imperatives(crate::ability_tree::imperative::ImperativeList),
    May(MayAbility),
    ReplacableImperatives(ReplacableImperatives),
}

impl crate::ability_tree::AbilityTreeNode for Statement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Statement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::Imperatives(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::May(ability) => abilities.push(ability as &dyn AbilityTreeNode),
            Self::ReplacableImperatives(ability) => abilities.push(ability as &dyn AbilityTreeNode),
        };
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "statement:")?;
        out.push_final_branch()?;
        match self {
            Statement::Imperatives(imp) => imp.display(out)?,
            Statement::May(may) => may.display(out)?,
            Statement::ReplacableImperatives(may) => may.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Statement {
    fn dummy_init() -> Self {
        Self::Imperatives(crate::utils::dummy())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct MayAbility {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub action: crate::ability_tree::imperative::Imperative,
    pub if_it_is_done: Option<Box<Statement>>,
    pub if_not_done: Option<Box<Statement>>,
}

impl crate::ability_tree::AbilityTreeNode for MayAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::MayAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn AbilityTreeNode);
        children.push(&self.action as &dyn AbilityTreeNode);
        match self.if_it_is_done.as_ref() {
            Some(child) => children.push(child.as_ref() as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        match self.if_not_done.as_ref() {
            Some(child) => children.push(child.as_ref() as &dyn AbilityTreeNode),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "may ability")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        self.player.display(out)?;
        out.next_inter_branch()?;
        write!(out, "may:")?;
        self.action.display(out)?;
        out.next_inter_branch()?;
        write!(out, "if it is done:")?;
        match self.if_it_is_done.as_ref() {
            Some(action) => action.display(out)?,
            None => write!(out, "none")?,
        }
        out.next_final_branch()?;
        write!(out, "if it is not done:")?;
        match self.if_not_done.as_ref() {
            Some(action) => action.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for MayAbility {
    fn dummy_init() -> Self {
        Self {
            player: crate::utils::dummy(),
            action: crate::utils::dummy(),
            if_it_is_done: None,
            if_not_done: None,
        }
    }
}

/// Replacable imperatives are a list of imperatives that can be replaced by another
/// list if some condition is met.
///
/// They always take the form: "do X. if Y, do Z instead".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ReplacableImperatives {
    pub first_clause: crate::ability_tree::imperative::ImperativeList,
    pub condition: crate::ability_tree::conditional::Conditional,
    pub replacing_clause: crate::ability_tree::imperative::ImperativeList,
}

impl crate::ability_tree::AbilityTreeNode for ReplacableImperatives {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ReplacableImperatives.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.condition as &dyn AbilityTreeNode);
        abilities.push(&self.first_clause as &dyn AbilityTreeNode);
        abilities.push(&self.replacing_clause as &dyn AbilityTreeNode);
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "replacable imperative")?;
        out.push_inter_branch()?;
        write!(out, "first imperative:")?;
        out.push_final_branch()?;
        self.first_clause.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "replacing condition:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "replacing imperative:")?;
        out.push_final_branch()?;
        self.replacing_clause.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ReplacableImperatives {
    fn dummy_init() -> Self {
        Self {
            first_clause: crate::utils::dummy(),
            condition: crate::utils::dummy(),
            replacing_clause: crate::utils::dummy(),
        }
    }
}
