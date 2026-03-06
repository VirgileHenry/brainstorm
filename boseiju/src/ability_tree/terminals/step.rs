use crate::lexer::IntoToken;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Step {
    Untap {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Upkeep {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Draw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BeginningOfCombat {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DeclareAttackers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DeclareBlockers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FirstStrikeDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Damage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LastStrikeDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EndOfCombat {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    End {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cleanup {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl Step {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Untap { span } => *span,
            Self::Upkeep { span } => *span,
            Self::Draw { span } => *span,
            Self::BeginningOfCombat { span } => *span,
            Self::DeclareAttackers { span } => *span,
            Self::DeclareBlockers { span } => *span,
            Self::FirstStrikeDamage { span } => *span,
            Self::Damage { span } => *span,
            Self::LastStrikeDamage { span } => *span,
            Self::EndOfCombat { span } => *span,
            Self::End { span } => *span,
            Self::Cleanup { span } => *span,
        }
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Untap { .. } => write!(f, "untap"),
            Step::Upkeep { .. } => write!(f, "upkeep"),
            Step::Draw { .. } => write!(f, "draw"),
            Step::BeginningOfCombat { .. } => write!(f, "beginning of combat"),
            Step::DeclareAttackers { .. } => write!(f, "declaration of attackers"),
            Step::DeclareBlockers { .. } => write!(f, "declaration of blockers"),
            Step::FirstStrikeDamage { .. } => write!(f, "first strike damage step"),
            Step::Damage { .. } => write!(f, "damage step"),
            Step::LastStrikeDamage { .. } => write!(f, "last strike damage step"),
            Step::EndOfCombat { .. } => write!(f, "end of combat"),
            Step::End { .. } => write!(f, "end step"),
            Step::Cleanup { .. } => write!(f, "cleanup"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for Step {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "untap step" => Some(Step::Untap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "upkeep" => Some(Step::Upkeep {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draw step" => Some(Step::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beginning of combat" => Some(Step::BeginningOfCombat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "declaration of attackers" => Some(Step::DeclareAttackers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "declaration of blockers" => Some(Step::DeclareBlockers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "first strike damage step" => Some(Step::FirstStrikeDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "damage step" => Some(Step::Damage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "last strike damage step" => Some(Step::LastStrikeDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end of combat" => Some(Step::EndOfCombat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end step" => Some(Step::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cleanup" => Some(Step::Cleanup {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
