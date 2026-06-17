use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::imperative_list::ImperativeList;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Imperatives(ImperativeList),
    May(MayAbility),
    ConditionalImperative(ConditionalImperative),
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
            Self::ConditionalImperative(ability) => abilities.push(ability as &dyn AbilityTreeNode),
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
            Statement::ConditionalImperative(may) => may.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "statement"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Imperatives(child) => child.node_span(),
            Self::May(child) => child.node_span(),
            Self::ConditionalImperative(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Statement {
    fn dummy_init() -> Self {
        Self::Imperatives(crate::utils::dummy())
    }
}

/// Fixme: own file
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayAbility {
    pub player: crate::ability_tree::player::PlayerSpecifier,
    pub action: crate::ability_tree::imperative_list::ImperativeList,
    pub if_it_is_done: Option<Box<Statement>>,
    pub if_not_done: Option<Box<Statement>>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
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

    fn node_tag(&self) -> &'static str {
        "may ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
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
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// Replacable imperatives are a list of imperatives that can be replaced by another
/// list if some condition is met.
///
/// They always take the form: "do X. if Y, do Z instead".
///
/// Fixme: own file
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalImperative {
    pub condition: crate::ability_tree::conditional::Conditional,
    pub condition_met_clause: crate::ability_tree::imperative_list::ImperativeList,
    pub cond_not_met_clause: Option<crate::ability_tree::imperative_list::ImperativeList>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for ConditionalImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ReplacableImperatives.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn AbilityTreeNode);
        children.push(&self.condition_met_clause as &dyn AbilityTreeNode);
        match self.cond_not_met_clause.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => {
                let dummy_node = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy_node as &dyn AbilityTreeNode)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "conditional imperative")?;
        out.push_inter_branch()?;
        write!(out, "condition:")?;
        out.push_final_branch()?;
        self.condition.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "if condition met imperative:")?;
        out.push_final_branch()?;
        self.condition_met_clause.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "if condition not met imperative:")?;
        out.push_final_branch()?;
        match self.cond_not_met_clause.as_ref() {
            Some(child) => child.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "conditional imperative"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionalImperative {
    fn dummy_init() -> Self {
        Self {
            condition_met_clause: crate::utils::dummy(),
            condition: crate::utils::dummy(),
            cond_not_met_clause: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
