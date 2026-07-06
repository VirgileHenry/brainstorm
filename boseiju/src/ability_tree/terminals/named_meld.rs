use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedMeld {
    HanweirTheWrithingTownship {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MishraLostToPhyrexia {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TitaniaGaeaIncarnate {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RagnarokDivineDeliverance {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChitteringHost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UrzaPlaneswalker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BriselaVoiceOfNightmares {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedMeld {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedMeldIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedMeld(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named meld"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::HanweirTheWrithingTownship { span } => *span,
            Self::MishraLostToPhyrexia { span } => *span,
            Self::TitaniaGaeaIncarnate { span } => *span,
            Self::RagnarokDivineDeliverance { span } => *span,
            Self::ChitteringHost { span } => *span,
            Self::UrzaPlaneswalker { span } => *span,
            Self::BriselaVoiceOfNightmares { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedMeld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedMeld::HanweirTheWrithingTownship { .. } => write!(f, "hanweir, the writhing township"),
            NamedMeld::MishraLostToPhyrexia { .. } => write!(f, "mishra, lost to phyrexia"),
            NamedMeld::TitaniaGaeaIncarnate { .. } => write!(f, "titania, gaea incarnate"),
            NamedMeld::RagnarokDivineDeliverance { .. } => write!(f, "ragnarok, divine deliverance"),
            NamedMeld::ChitteringHost { .. } => write!(f, "chittering host"),
            NamedMeld::UrzaPlaneswalker { .. } => write!(f, "urza, planeswalker"),
            NamedMeld::BriselaVoiceOfNightmares { .. } => write!(f, "brisela, voice of nightmares"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedMeld {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "hanweir, the writhing township" => Some(NamedMeld::HanweirTheWrithingTownship {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mishra, lost to phyrexia" => Some(NamedMeld::MishraLostToPhyrexia {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "titania, gaea incarnate" => Some(NamedMeld::TitaniaGaeaIncarnate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ragnarok, divine deliverance" => Some(NamedMeld::RagnarokDivineDeliverance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chittering host" => Some(NamedMeld::ChitteringHost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "urza, planeswalker" => Some(NamedMeld::UrzaPlaneswalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brisela, voice of nightmares" => Some(NamedMeld::BriselaVoiceOfNightmares {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
