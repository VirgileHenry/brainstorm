use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: state specific based on kind ? spells can be countered,
/// creatures can be attacking...
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardState {
    Attached {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Attacking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enchanted {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Equipped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Exiled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Revealed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Untapped {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl CardState {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Attached {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }
}

impl AbilityTreeNode for CardState {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::CardStateIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::CardState(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "owner specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attached { span } => *span,
            Self::Attacking { span } => *span,
            Self::Blocking { span } => *span,
            Self::Blocked { span } => *span,
            Self::Enchanted { span } => *span,
            Self::Equipped { span } => *span,
            Self::Exiled { span } => *span,
            Self::Revealed { span } => *span,
            Self::Tapped { span } => *span,
            Self::Untapped { span } => *span,
        }
    }
}

impl std::fmt::Display for CardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardState::Attached { .. } => write!(f, "attached"),
            CardState::Attacking { .. } => write!(f, "attacking"),
            CardState::Blocking { .. } => write!(f, "blocking"),
            CardState::Blocked { .. } => write!(f, "blocked"),
            CardState::Enchanted { .. } => write!(f, "enchanted"),
            CardState::Equipped { .. } => write!(f, "equipped"),
            CardState::Exiled { .. } => write!(f, "exiled"),
            CardState::Revealed { .. } => write!(f, "exiled"),
            CardState::Tapped { .. } => write!(f, "tapped"),
            CardState::Untapped { .. } => write!(f, "untapped"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for CardState {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "attached" => Some(CardState::Attached {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacking" => Some(CardState::Attacking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocking" => Some(CardState::Blocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blocked" => Some(CardState::Blocked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enchanted" => Some(CardState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "equipped" => Some(CardState::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exiled" => Some(CardState::Exiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "revealed" => Some(CardState::Revealed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tapped" => Some(CardState::Tapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "untapped" => Some(CardState::Untapped {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CardState {
    fn dummy_init() -> Self {
        Self::Attacking {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
