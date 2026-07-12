use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Imperative to create tokens.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTokenImperative {
    pub tokens: crate::HeapArrayVec<TokenCreation, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CreateTokenImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::DealsDamageImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.tokens.iter() {
            children.push(child as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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

    fn node_tag(&self) -> &'static str {
        "create tokens imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreateTokenImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreateTokenImperative {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A kind of created token as well as an amount for this token.
///
/// This node regroups a group of created tokens, e.g. "3 1/1 red goblins".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenCreation {
    pub amount: crate::ability_tree::number::Number,
    pub token: CreatedTokenKind,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for TokenCreation {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::TokenCreation.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children.push(&self.token as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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

    fn node_tag(&self) -> &'static str {
        "token creation"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TokenCreation {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

/// The kind of token that we shall create with a token creation imperative.
///
/// This will either be a given token kind, e.g. "create a 2/2 red warrior token"
/// or will reference a token previously mentionned in the ability, e.g.
/// "create twice as many of those tokens instead".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatedTokenKind {
    PreviouslyMentionnedToken {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NewToken(crate::card::layout::token::TokenLayout),
}

impl Node for CreatedTokenKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ReplacedTokenKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NewToken(child) => children.push(child as &dyn Node),
            Self::PreviouslyMentionnedToken { .. } => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::NodeKind::PreviouslyMentionnedToken.id(),
            ) as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "token kind:")?;
        out.push_final_branch()?;
        match self {
            Self::PreviouslyMentionnedToken { .. } => write!(out, "previously mentionned token")?,
            Self::NewToken(token) => token.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "token kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatedTokenKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::PreviouslyMentionnedToken { span } => *span,
            Self::NewToken(child) => child.span(),
        }
    }
}

impl Default for CreatedTokenKind {
    fn default() -> Self {
        Self::PreviouslyMentionnedToken {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
