use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Phase {
    Beginning {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PrecombatMain {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Combat {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PostcombatMain {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    End {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Current {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for Phase {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::PhaseIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::Phase(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "phase"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Beginning { span } => *span,
            Self::PrecombatMain { span } => *span,
            Self::Combat { span } => *span,
            Self::PostcombatMain { span } => *span,
            Self::End { span } => *span,
            Self::Current { span } => *span,
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Beginning { .. } => write!(f, "beginning phase"),
            Phase::PrecombatMain { .. } => write!(f, "precombat main phase"),
            Phase::Combat { .. } => write!(f, "combat phase"),
            Phase::PostcombatMain { .. } => write!(f, "postcombat main phase"),
            Phase::End { .. } => write!(f, "end phase"),
            Phase::Current { .. } => write!(f, "this phase"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for Phase {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "beginning phase" => Some(Phase::Beginning {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "precombat main phase" => Some(Phase::PrecombatMain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combat phase" | "combat" => Some(Phase::Combat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "postcombat main phase" => Some(Phase::PostcombatMain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end phase" => Some(Phase::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end of turn" => Some(Phase::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this phase" => Some(Phase::Current {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Phase {
    fn dummy_init() -> Self {
        Self::Beginning {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
