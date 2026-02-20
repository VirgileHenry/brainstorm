use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

const MAX_CREATED_TOKEN: usize = MAX_CHILDREN_PER_NODE - 1;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenCreationReplacement {
    source_ref: super::source_ref::EventSourceReference,
    /* Fixme: replace with create token imperative here */
    tokens: arrayvec::ArrayVec<TokenCreation, MAX_CREATED_TOKEN>,
}

impl AbilityTreeNode for TokenCreationReplacement {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TokenCreationReplacement.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.source_ref as &dyn AbilityTreeNode);
        for token in self.tokens.iter() {
            children.push(token as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "token creation replacement:")?;
        out.push_inter_branch()?;
        write!(out, "effect source:")?;
        out.push_final_branch()?;
        self.source_ref.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "created tokens:")?;
        for (i, token) in self.tokens.iter().enumerate() {
            if i == self.tokens.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            token.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenCreation {
    pub amount: crate::ability_tree::number::Number,
    pub token: ReplacedTokenKind,
}

impl AbilityTreeNode for TokenCreation {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TokenCreation.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn AbilityTreeNode);
        children.push(&self.token as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "create token:")?;
        out.push_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "of token:")?;
        out.push_final_branch()?;
        self.token.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ReplacedTokenKind {
    PreviouslyMentionnedToken,
    NewToken(Box<crate::ability_tree::card_layout::TokenLayout>),
}

impl AbilityTreeNode for ReplacedTokenKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ReplacedTokenKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NewToken(child) => children.push(child.as_ref() as &dyn AbilityTreeNode),
            Self::PreviouslyMentionnedToken => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::PreviouslyMentionnedToken.id(),
            ) as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "replaced token kind:")?;
        out.push_final_branch()?;
        match self {
            Self::PreviouslyMentionnedToken => write!(out, "previously mentionned token")?,
            Self::NewToken(token) => token.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
