#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Step {
    Untap {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Upkeep {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Draw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BeginningOfCombat {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DeclareAttackers {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DeclareBlockers {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FirstStrikeDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Damage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LastStrikeDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    EndOfCombat {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    End {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cleanup {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for Step {
    fn default() -> Self {
        Self::Untap {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Step {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Step {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "untap step" | "untap steps" => Ok(Step::Untap {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "upkeep" | "upkeeps" => Ok(Step::Upkeep {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draw step" | "draw steps" => Ok(Step::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beginning of combat" => Ok(Step::BeginningOfCombat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "declare attackers step" | "declare attackers steps" | "attackers are declared" => Ok(Step::DeclareAttackers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "declare blockers step" | "declare blockers steps" | "blockers are declared" | "declaring blockers" => {
                Ok(Step::DeclareBlockers {
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })
            }
            "first strike damage step" => Ok(Step::FirstStrikeDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "damage step" => Ok(Step::Damage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end of combat" => Ok(Step::EndOfCombat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end step" | "end steps" => Ok(Step::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cleanup" => Ok(Step::Cleanup {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
