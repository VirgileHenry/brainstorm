use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedTransformation {
    Fenric {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HumbleMerchant {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LegitimateBuisnessperson {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedTransformation {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedTransformationIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedTransformation(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named transformation"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Fenric { span } => *span,
            Self::HumbleMerchant { span } => *span,
            Self::LegitimateBuisnessperson { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedTransformation::Fenric { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::HumbleMerchant { .. } => write!(f, "legitimate businessperson"),
            NamedTransformation::LegitimateBuisnessperson { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedTransformation {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "fenric" => Some(NamedTransformation::Fenric {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "humble merchant" => Some(NamedTransformation::HumbleMerchant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legitimate businessperson" => Some(NamedTransformation::LegitimateBuisnessperson {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
