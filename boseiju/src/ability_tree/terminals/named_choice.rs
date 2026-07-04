use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedChoice {
    Brotherhood {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enclave {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Foe {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Friend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Khans {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedChoice {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedChoiceIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedChoice(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named choice"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Brotherhood { span } => *span,
            Self::Enclave { span } => *span,
            Self::Foe { span } => *span,
            Self::Friend { span } => *span,
            Self::Khans { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedChoice::Brotherhood { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Enclave { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Foe { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Friend { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Khans { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedChoice {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "brotherhood" => Some(NamedChoice::Brotherhood {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enclave" => Some(NamedChoice::Enclave {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "foe" => Some(NamedChoice::Foe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friend" => Some(NamedChoice::Friend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khans" => Some(NamedChoice::Khans {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
