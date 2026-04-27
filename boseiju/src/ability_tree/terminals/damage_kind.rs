use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageKind {
    CombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Damage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NoncombatDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl DamageKind {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::CombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::Damage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
    }
}

impl AbilityTreeNode for DamageKind {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::DamageKindIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::DamageKind(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "damage kind")?;
        out.push_final_branch()?;
        write!(out, "{self}")?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "damage kind"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::CombatDamage { span } => *span,
            Self::Damage { span } => *span,
            Self::NoncombatDamage { span } => *span,
        }
    }
}

impl std::fmt::Display for DamageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CombatDamage { .. } => write!(f, "combat damage"),
            Self::Damage { .. } => write!(f, "any damage"),
            Self::NoncombatDamage { .. } => write!(f, "non combat damage"),
        }
    }
}

impl IntoToken for DamageKind {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "damage" | "damages" => Some(Self::Damage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combat damage" => Some(Self::CombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "noncombat damage" => Some(Self::NoncombatDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for DamageKind {
    fn dummy_init() -> Self {
        Self::Damage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
