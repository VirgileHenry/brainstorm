use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnerSpecifier {
    YouOwn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YouDontOwn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ObjectOwner {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl OwnerSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::YouOwn { span } => *span,
            Self::YouDontOwn { span } => *span,
            Self::ObjectOwner { span } => *span,
        }
    }
}

impl AbilityTreeNode for OwnerSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::OwnerSpecifierIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::OwnerSpecifier(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::YouOwn { span } => *span,
            Self::YouDontOwn { span } => *span,
            Self::ObjectOwner { span } => *span,
        }
    }
}

impl std::fmt::Display for OwnerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerSpecifier::YouOwn { .. } => write!(f, "you own"),
            OwnerSpecifier::YouDontOwn { .. } => write!(f, "you don't own"),
            OwnerSpecifier::ObjectOwner { .. } => write!(f, "its owner"),
        }
    }
}

impl IntoToken for OwnerSpecifier {
    #[cfg(feature = "lexer")]
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "you own" | "your" => Some(OwnerSpecifier::YouOwn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you don't own" => Some(OwnerSpecifier::YouDontOwn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "its owner" => Some(OwnerSpecifier::ObjectOwner {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for OwnerSpecifier {
    fn dummy_init() -> Self {
        Self::YouOwn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
