use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedChoice {
    Abzan {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Believe {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Brotherhood {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Doubt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enclave {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Foe {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fortune {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Friend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Friends {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Jeskai {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Khans {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Legion {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mardu {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mirran {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Money {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ncr {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Peace {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Phyrexian {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Secrets {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Silence {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Snitch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Sultai {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Temur {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    War {
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
            Self::Abzan { span } => *span,
            Self::Believe { span } => *span,
            Self::Brotherhood { span } => *span,
            Self::Doubt { span } => *span,
            Self::Enclave { span } => *span,
            Self::Fame { span } => *span,
            Self::Foe { span } => *span,
            Self::Fortune { span } => *span,
            Self::Friend { span } => *span,
            Self::Friends { span } => *span,
            Self::Jeskai { span } => *span,
            Self::Khans { span } => *span,
            Self::Legion { span } => *span,
            Self::Mardu { span } => *span,
            Self::Mirran { span } => *span,
            Self::Money { span } => *span,
            Self::Ncr { span } => *span,
            Self::Peace { span } => *span,
            Self::Phyrexian { span } => *span,
            Self::Secrets { span } => *span,
            Self::Silence { span } => *span,
            Self::Snitch { span } => *span,
            Self::Sultai { span } => *span,
            Self::Temur { span } => *span,
            Self::War { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedChoice::Abzan { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Believe { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Brotherhood { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Doubt { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Enclave { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Fame { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Foe { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Fortune { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Friend { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Friends { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Jeskai { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Khans { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Legion { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Mardu { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Mirran { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Money { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Ncr { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Peace { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Phyrexian { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Secrets { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Silence { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Snitch { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Sultai { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::Temur { .. } => write!(f, "legitimate businessperson"),
            NamedChoice::War { .. } => write!(f, "legitimate businessperson"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedChoice {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "abzan" => Some(NamedChoice::Abzan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "believe" => Some(NamedChoice::Believe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brotherhood" => Some(NamedChoice::Brotherhood {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "doubt" => Some(NamedChoice::Doubt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enclave" => Some(NamedChoice::Enclave {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fame" => Some(NamedChoice::Fame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "foe" => Some(NamedChoice::Foe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fortune" => Some(NamedChoice::Fortune {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friend" => Some(NamedChoice::Friend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "friends" => Some(NamedChoice::Friend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jeskai" => Some(NamedChoice::Jeskai {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khans" => Some(NamedChoice::Khans {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legion" => Some(NamedChoice::Legion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mardu" => Some(NamedChoice::Mardu {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mirran" => Some(NamedChoice::Mirran {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "money" => Some(NamedChoice::Money {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ncr" => Some(NamedChoice::Ncr {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "peace" => Some(NamedChoice::Peace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phyrexian" => Some(NamedChoice::Phyrexian {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "secrets" => Some(NamedChoice::Secrets {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silence" => Some(NamedChoice::Silence {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "snitch" => Some(NamedChoice::Snitch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sultai" => Some(NamedChoice::Sultai {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "temur" => Some(NamedChoice::Temur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "war" => Some(NamedChoice::War {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
