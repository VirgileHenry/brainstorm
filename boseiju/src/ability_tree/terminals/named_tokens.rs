use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedToken {
    KomasCoil {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedToken {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::NamedTokenIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedToken(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named token"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::KomasCoil { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedToken {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "koma's coil" => Some(Self::KomasCoil {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "~'s coil" => Some(Self::KomasCoil {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }), /* Weird case for koma creating koma's coil */
            _ => None,
        }
    }
}

impl std::fmt::Display for NamedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KomasCoil { .. } => write!(f, "koma's coil"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for NamedToken {
    fn dummy_init() -> Self {
        Self::KomasCoil {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
