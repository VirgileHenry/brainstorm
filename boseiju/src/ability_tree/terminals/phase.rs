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

#[cfg(feature = "spanned_tree")]
impl Phase {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
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

impl IntoToken for Phase {
    #[cfg(feature = "lexer")]
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
