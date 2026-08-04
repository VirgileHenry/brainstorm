use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::imperative_list::ImperativeList;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Imperatives(ImperativeList),
    May(MayAbility),
    ConditionalImperative(ConditionalImperative),
}

impl crate::Node for Statement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Statement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        match self {
            Self::Imperatives(ability) => abilities.push(ability as &dyn Node),
            Self::May(ability) => abilities.push(ability as &dyn Node),
            Self::ConditionalImperative(ability) => abilities.push(ability as &dyn Node),
        };
        abilities
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Statement {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Imperatives(child) => child.span(),
            Self::May(child) => child.span(),
            Self::ConditionalImperative(child) => child.span(),
        }
    }
}

impl Default for Statement {
    fn default() -> Self {
        Self::Imperatives(Default::default())
    }
}

/// Fixme: own file
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayAbility {
    pub player: crate::ability_tree::player::PlayerReference,
    pub action: crate::ability_tree::imperative_list::ImperativeList,
    pub if_it_is_done: Option<Box<Statement>>,
    pub if_not_done: Option<Box<Statement>>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for MayAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::MayAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        children.push(&self.action as &dyn Node);
        match self.if_it_is_done.as_ref() {
            Some(child) => children.push(child.as_ref() as &dyn Node),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        match self.if_not_done.as_ref() {
            Some(child) => children.push(child.as_ref() as &dyn Node),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for MayAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for MayAbility {
    fn default() -> Self {
        Self {
            player: Default::default(),
            action: Default::default(),
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
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionalImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ReplacableImperatives.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.condition as &dyn Node);
        children.push(&self.condition_met_clause as &dyn Node);
        match self.cond_not_met_clause.as_ref() {
            Some(child) => children.push(child as &dyn Node),
            None => {
                let dummy_node = crate::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(dummy_node as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionalImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionalImperative {
    fn default() -> Self {
        Self {
            condition_met_clause: Default::default(),
            condition: Default::default(),
            cond_not_met_clause: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
