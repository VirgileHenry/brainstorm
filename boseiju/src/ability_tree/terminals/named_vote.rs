use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedVotes {
    Aid {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bribery {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Carnage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Condemnation {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Consequences {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Death {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Denial {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Dominion {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Duplication {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Embark {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Evidence {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Feather {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fellowship {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Free {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Grace {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Guidance {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Guilty {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Harvest {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Homage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Innocent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Knowledge {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Money {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Nah {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Numbers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Past {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Planeswalk {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Present {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Profit {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Psychosis {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Quill {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Security {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Sickness {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Sprout {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Strength {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Taxes {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Time {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Torture {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Truth {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Wild {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Yeah {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedVotes {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedVotesIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedVotes(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named vote"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Aid { span } => *span,
            Self::Bribery { span } => *span,
            Self::Carnage { span } => *span,
            Self::Condemnation { span } => *span,
            Self::Consequences { span } => *span,
            Self::Death { span } => *span,
            Self::Denial { span } => *span,
            Self::Dominion { span } => *span,
            Self::Duplication { span } => *span,
            Self::Embark { span } => *span,
            Self::Evidence { span } => *span,
            Self::Feather { span } => *span,
            Self::Fellowship { span } => *span,
            Self::Free { span } => *span,
            Self::Grace { span } => *span,
            Self::Guidance { span } => *span,
            Self::Guilty { span } => *span,
            Self::Harvest { span } => *span,
            Self::Homage { span } => *span,
            Self::Innocent { span } => *span,
            Self::Knowledge { span } => *span,
            Self::Money { span } => *span,
            Self::Nah { span } => *span,
            Self::Numbers { span } => *span,
            Self::Past { span } => *span,
            Self::Planeswalk { span } => *span,
            Self::Present { span } => *span,
            Self::Profit { span } => *span,
            Self::Psychosis { span } => *span,
            Self::Quill { span } => *span,
            Self::Security { span } => *span,
            Self::Sickness { span } => *span,
            Self::Sprout { span } => *span,
            Self::Strength { span } => *span,
            Self::Taxes { span } => *span,
            Self::Time { span } => *span,
            Self::Torture { span } => *span,
            Self::Truth { span } => *span,
            Self::Wild { span } => *span,
            Self::Yeah { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedVotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedVotes::Aid { .. } => write!(f, "aid"),
            NamedVotes::Bribery { .. } => write!(f, "bribery"),
            NamedVotes::Carnage { .. } => write!(f, "carnage"),
            NamedVotes::Condemnation { .. } => write!(f, "condemnation"),
            NamedVotes::Consequences { .. } => write!(f, "consequences"),
            NamedVotes::Death { .. } => write!(f, "death"),
            NamedVotes::Denial { .. } => write!(f, "denial"),
            NamedVotes::Dominion { .. } => write!(f, "dominion"),
            NamedVotes::Duplication { .. } => write!(f, "duplication"),
            NamedVotes::Embark { .. } => write!(f, "embark"),
            NamedVotes::Evidence { .. } => write!(f, "evidence"),
            NamedVotes::Feather { .. } => write!(f, "feather"),
            NamedVotes::Fellowship { .. } => write!(f, "fellowship"),
            NamedVotes::Free { .. } => write!(f, "free"),
            NamedVotes::Grace { .. } => write!(f, "grace"),
            NamedVotes::Guidance { .. } => write!(f, "guidance"),
            NamedVotes::Guilty { .. } => write!(f, "guilty"),
            NamedVotes::Harvest { .. } => write!(f, "harvest"),
            NamedVotes::Homage { .. } => write!(f, "homage"),
            NamedVotes::Innocent { .. } => write!(f, "innocent"),
            NamedVotes::Knowledge { .. } => write!(f, "knowledge"),
            NamedVotes::Money { .. } => write!(f, "money"),
            NamedVotes::Nah { .. } => write!(f, "nah"),
            NamedVotes::Numbers { .. } => write!(f, "numbers"),
            NamedVotes::Past { .. } => write!(f, "past"),
            NamedVotes::Planeswalk { .. } => write!(f, "planeswalk"),
            NamedVotes::Present { .. } => write!(f, "present"),
            NamedVotes::Profit { .. } => write!(f, "profit"),
            NamedVotes::Psychosis { .. } => write!(f, "psychosis"),
            NamedVotes::Quill { .. } => write!(f, "quill"),
            NamedVotes::Security { .. } => write!(f, "security"),
            NamedVotes::Sickness { .. } => write!(f, "sickness"),
            NamedVotes::Sprout { .. } => write!(f, "sprout"),
            NamedVotes::Strength { .. } => write!(f, "strength"),
            NamedVotes::Taxes { .. } => write!(f, "taxes"),
            NamedVotes::Time { .. } => write!(f, "time"),
            NamedVotes::Torture { .. } => write!(f, "torture"),
            NamedVotes::Truth { .. } => write!(f, "truth"),
            NamedVotes::Wild { .. } => write!(f, "wild"),
            NamedVotes::Yeah { .. } => write!(f, "yeah"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedVotes {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "aid" => Some(NamedVotes::Aid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bribery" => Some(NamedVotes::Bribery {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "carnage" => Some(NamedVotes::Carnage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "condemnation" => Some(NamedVotes::Condemnation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "consequences" => Some(NamedVotes::Consequences {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "death" => Some(NamedVotes::Death {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "denial" => Some(NamedVotes::Denial {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "dominion" => Some(NamedVotes::Dominion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "duplication" => Some(NamedVotes::Duplication {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "embark" => Some(NamedVotes::Embark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evidence" => Some(NamedVotes::Evidence {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "feather" => Some(NamedVotes::Feather {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fellowship" => Some(NamedVotes::Fellowship {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "free" => Some(NamedVotes::Free {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "grace" => Some(NamedVotes::Grace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guidance" => Some(NamedVotes::Guidance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guilty" => Some(NamedVotes::Guilty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "harvest" => Some(NamedVotes::Harvest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "homage" => Some(NamedVotes::Homage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "innocent" => Some(NamedVotes::Innocent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "knowledge" => Some(NamedVotes::Knowledge {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "money" => Some(NamedVotes::Money {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nah" => Some(NamedVotes::Nah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "numbers" => Some(NamedVotes::Numbers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "past" => Some(NamedVotes::Past {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planeswalk" => Some(NamedVotes::Planeswalk {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "present" => Some(NamedVotes::Present {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "profit" => Some(NamedVotes::Profit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "psychosis" => Some(NamedVotes::Psychosis {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quill" => Some(NamedVotes::Quill {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "security" => Some(NamedVotes::Security {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sickness" => Some(NamedVotes::Sickness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sprout" => Some(NamedVotes::Sprout {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "strength" => Some(NamedVotes::Strength {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "taxes" => Some(NamedVotes::Taxes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "time" => Some(NamedVotes::Time {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "torture" => Some(NamedVotes::Torture {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "truth" => Some(NamedVotes::Truth {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wild" => Some(NamedVotes::Wild {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yeah" => Some(NamedVotes::Yeah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
