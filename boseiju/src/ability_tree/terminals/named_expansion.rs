use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedExpansion {
    Antiquities {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Homelands {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedExpansion {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedExpansionIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedExpansion(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named extension"
    }
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for NamedExpansion {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Antiquities { span } => *span,
            Self::Homelands { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedExpansion::Antiquities { .. } => write!(f, "legitimate businessperson"),
            NamedExpansion::Homelands { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedExpansion {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "antiquities expansion" => Some(NamedExpansion::Antiquities {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "homelands expansion" => Some(NamedExpansion::Homelands {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
