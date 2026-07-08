use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedVote {
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
    MinesOfMoria {
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
    RedhornPass {
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

impl AbilityTreeNode for NamedVote {
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
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for NamedVote {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
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
            Self::MinesOfMoria { span } => *span,
            Self::Money { span } => *span,
            Self::Nah { span } => *span,
            Self::Numbers { span } => *span,
            Self::Past { span } => *span,
            Self::Planeswalk { span } => *span,
            Self::Present { span } => *span,
            Self::Profit { span } => *span,
            Self::Psychosis { span } => *span,
            Self::Quill { span } => *span,
            Self::RedhornPass { span } => *span,
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

impl std::fmt::Display for NamedVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedVote::Aid { .. } => write!(f, "aid"),
            NamedVote::Bribery { .. } => write!(f, "bribery"),
            NamedVote::Carnage { .. } => write!(f, "carnage"),
            NamedVote::Condemnation { .. } => write!(f, "condemnation"),
            NamedVote::Consequences { .. } => write!(f, "consequences"),
            NamedVote::Death { .. } => write!(f, "death"),
            NamedVote::Denial { .. } => write!(f, "denial"),
            NamedVote::Dominion { .. } => write!(f, "dominion"),
            NamedVote::Duplication { .. } => write!(f, "duplication"),
            NamedVote::Embark { .. } => write!(f, "embark"),
            NamedVote::Evidence { .. } => write!(f, "evidence"),
            NamedVote::Feather { .. } => write!(f, "feather"),
            NamedVote::Fellowship { .. } => write!(f, "fellowship"),
            NamedVote::Free { .. } => write!(f, "free"),
            NamedVote::Grace { .. } => write!(f, "grace"),
            NamedVote::Guidance { .. } => write!(f, "guidance"),
            NamedVote::Guilty { .. } => write!(f, "guilty"),
            NamedVote::Harvest { .. } => write!(f, "harvest"),
            NamedVote::Homage { .. } => write!(f, "homage"),
            NamedVote::Innocent { .. } => write!(f, "innocent"),
            NamedVote::Knowledge { .. } => write!(f, "knowledge"),
            NamedVote::MinesOfMoria { .. } => write!(f, "mines of moria"),
            NamedVote::Money { .. } => write!(f, "money"),
            NamedVote::Nah { .. } => write!(f, "nah"),
            NamedVote::Numbers { .. } => write!(f, "numbers"),
            NamedVote::Past { .. } => write!(f, "past"),
            NamedVote::Planeswalk { .. } => write!(f, "planeswalk"),
            NamedVote::Present { .. } => write!(f, "present"),
            NamedVote::Profit { .. } => write!(f, "profit"),
            NamedVote::Psychosis { .. } => write!(f, "psychosis"),
            NamedVote::Quill { .. } => write!(f, "quill"),
            NamedVote::RedhornPass { .. } => write!(f, "readhorn pass"),
            NamedVote::Security { .. } => write!(f, "security"),
            NamedVote::Sickness { .. } => write!(f, "sickness"),
            NamedVote::Sprout { .. } => write!(f, "sprout"),
            NamedVote::Strength { .. } => write!(f, "strength"),
            NamedVote::Taxes { .. } => write!(f, "taxes"),
            NamedVote::Time { .. } => write!(f, "time"),
            NamedVote::Torture { .. } => write!(f, "torture"),
            NamedVote::Truth { .. } => write!(f, "truth"),
            NamedVote::Wild { .. } => write!(f, "wild"),
            NamedVote::Yeah { .. } => write!(f, "yeah"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedVote {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "aid" => Some(NamedVote::Aid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bribery" => Some(NamedVote::Bribery {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "carnage" => Some(NamedVote::Carnage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "condemnation" => Some(NamedVote::Condemnation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "consequences" => Some(NamedVote::Consequences {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "death" => Some(NamedVote::Death {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "denial" => Some(NamedVote::Denial {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "dominion" => Some(NamedVote::Dominion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "duplication" => Some(NamedVote::Duplication {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "embark" => Some(NamedVote::Embark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evidence" => Some(NamedVote::Evidence {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "feather" => Some(NamedVote::Feather {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fellowship" => Some(NamedVote::Fellowship {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "free" => Some(NamedVote::Free {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "grace" => Some(NamedVote::Grace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guidance" => Some(NamedVote::Guidance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guilty" => Some(NamedVote::Guilty {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "harvest" => Some(NamedVote::Harvest {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "homage" => Some(NamedVote::Homage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "innocent" => Some(NamedVote::Innocent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "knowledge" => Some(NamedVote::Knowledge {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mines of moria" => Some(NamedVote::MinesOfMoria {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "money" => Some(NamedVote::Money {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nah" => Some(NamedVote::Nah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "numbers" => Some(NamedVote::Numbers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "past" => Some(NamedVote::Past {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planeswalk" => Some(NamedVote::Planeswalk {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "present" => Some(NamedVote::Present {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "profit" => Some(NamedVote::Profit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "psychosis" => Some(NamedVote::Psychosis {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "quill" => Some(NamedVote::Quill {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "redhorn pass" => Some(NamedVote::RedhornPass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "security" => Some(NamedVote::Security {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sickness" => Some(NamedVote::Sickness {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sprout" => Some(NamedVote::Sprout {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "strength" => Some(NamedVote::Strength {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "taxes" => Some(NamedVote::Taxes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "time" => Some(NamedVote::Time {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "torture" => Some(NamedVote::Torture {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "truth" => Some(NamedVote::Truth {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wild" => Some(NamedVote::Wild {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yeah" => Some(NamedVote::Yeah {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
