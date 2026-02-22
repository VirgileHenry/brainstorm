use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Imperative to create tokens.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CreateTokenImperative {
    pub tokens: arrayvec::ArrayVec<TokenCreation, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for CreateTokenImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::DealsDamageImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.tokens.iter() {
            children.push(child as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "create tokens:")?;
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

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreateTokenImperative {
    fn dummy_init() -> Self {
        Self {
            tokens: crate::utils::dummy(),
        }
    }
}

/// A kind of created token as well as an amount for this token.
///
/// This node regroups a group of created tokens, e.g. "3 1/1 red goblins".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenCreation {
    pub amount: crate::ability_tree::number::Number,
    pub token: CreatedTokenKind,
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

/// The kind of token that we shall create with a token creation imperative.
///
/// This will either be a given token kind, e.g. "create a 2/2 red warrior token"
/// or will reference a token previously mentionned in the ability, e.g.
/// "create twice as many of those tokens instead".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CreatedTokenKind {
    PreviouslyMentionnedToken,
    NewToken(Box<crate::ability_tree::card_layout::TokenLayout>),
}

impl AbilityTreeNode for CreatedTokenKind {
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
        write!(out, "token kind:")?;
        out.push_final_branch()?;
        match self {
            Self::PreviouslyMentionnedToken => write!(out, "previously mentionned token")?,
            Self::NewToken(token) => token.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatedTokenKind {
    fn dummy_init() -> Self {
        Self::PreviouslyMentionnedToken
    }
}
