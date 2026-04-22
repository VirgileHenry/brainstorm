use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardProperty {
    BasePower {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BasePowerAndToughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BaseToughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Color {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ManaValue {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Name {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Power {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Toughness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for CardProperty {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::CardPropertyIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::CardProperty(*self)).id();
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
            Self::BasePower { span } => *span,
            Self::BasePowerAndToughness { span } => *span,
            Self::BaseToughness { span } => *span,
            Self::Color { span } => *span,
            Self::ManaValue { span } => *span,
            Self::Name { span } => *span,
            Self::Power { span } => *span,
            Self::Toughness { span } => *span,
        }
    }
}

impl std::fmt::Display for CardProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardProperty::BasePower { .. } => write!(f, "base power"),
            CardProperty::BasePowerAndToughness { .. } => write!(f, "base power and toughness"),
            CardProperty::BaseToughness { .. } => write!(f, "base toughness"),
            CardProperty::Color { .. } => write!(f, "color"),
            CardProperty::ManaValue { .. } => write!(f, "mana value"),
            CardProperty::Name { .. } => write!(f, "name"),
            CardProperty::Power { .. } => write!(f, "power"),
            CardProperty::Toughness { .. } => write!(f, "touhness"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for CardProperty {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "base power" => Some(CardProperty::BasePower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base power and toughness" => Some(CardProperty::BasePowerAndToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "base toughness" => Some(CardProperty::BaseToughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana cost" | "mana value" => Some(CardProperty::ManaValue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "name" => Some(CardProperty::Name {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "power" => Some(CardProperty::Power {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toughness" => Some(CardProperty::Toughness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
