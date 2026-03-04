use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Control specifier for objects.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlSpecifier {
    YouControl {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YouDontControl {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl ControlSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::YouControl { span } => *span,
            Self::YouDontControl { span } => *span,
        }
    }
}

impl AbilityTreeNode for ControlSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::ControlSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::ControlSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "control specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::YouControl { span } => *span,
            Self::YouDontControl { span } => *span,
        }
    }
}

impl std::fmt::Display for ControlSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YouControl { .. } => write!(f, "you control"),
            Self::YouDontControl { .. } => write!(f, "you don't control"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for ControlSpecifier {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "you control" | "you already control" => Some(Self::YouControl {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you don't control" | "your opponents control" | "an opponent controls" => Some(Self::YouDontControl {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
