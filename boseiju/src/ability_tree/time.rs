mod incoming_instant;
mod recurrent_instant;
mod step_or_phase;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

pub use incoming_instant::*;
pub use recurrent_instant::*;
pub use step_or_phase::*;

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instant {
    Incoming(IncomingInstant),
    Reccurent(RecurrentInstant),
}

impl AbilityTreeNode for Instant {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Instant.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Incoming(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Reccurent(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "instant:")?;
        out.push_final_branch()?;
        match self {
            Self::Incoming(child) => child.display(out)?,
            Self::Reccurent(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "instant"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Incoming(child) => child.node_span(),
            Self::Reccurent(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Instant {
    fn dummy_init() -> Self {
        Self::Reccurent(crate::utils::dummy())
    }
}

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ForwardDuration {
    ForAsLongAsItsExiled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Forever {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UntilEndOfTurn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UntilEndOfYourNextTurn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for ForwardDuration {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::ForwardDurationIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::ForwardDuration(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn node_tag(&self) -> &'static str {
        "forward duration"
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ForAsLongAsItsExiled { span } => *span,
            Self::Forever { span } => *span,
            Self::UntilEndOfTurn { span } => *span,
            Self::UntilEndOfYourNextTurn { span } => *span,
        }
    }
}

impl IntoToken for ForwardDuration {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "until end of turn" => Some(Self::UntilEndOfTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            "until the end of your next turn" => Some(Self::UntilEndOfYourNextTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ForwardDuration {
    fn dummy_init() -> Self {
        Self::UntilEndOfTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl std::fmt::Display for ForwardDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForAsLongAsItsExiled { .. } => write!(f, "for as long as it remains exiled"),
            Self::Forever { .. } => write!(f, "forever"),
            Self::UntilEndOfTurn { .. } => write!(f, "until end of turn"),
            Self::UntilEndOfYourNextTurn { .. } => write!(f, "until the end of your next turn"),
        }
    }
}

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackwardDuration {
    ThisTurn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for BackwardDuration {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::BackwardDurationIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::BackwardDuration(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "backward duration"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::ThisTurn { span } => *span,
        }
    }
}

impl IntoToken for BackwardDuration {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "this turn" => Some(Self::ThisTurn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for BackwardDuration {
    fn dummy_init() -> Self {
        Self::ThisTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl std::fmt::Display for BackwardDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThisTurn { .. } => write!(f, "this turn"),
        }
    }
}
