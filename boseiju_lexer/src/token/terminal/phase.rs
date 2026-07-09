#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Phase {
    Beginning {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Combat {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Current {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    End {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MainPhase {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PostcombatMain {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PrecombatMain {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Phase {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Beginning { span } => *span,
            Self::Combat { span } => *span,
            Self::Current { span } => *span,
            Self::End { span } => *span,
            Self::MainPhase { span } => *span,
            Self::PostcombatMain { span } => *span,
            Self::PrecombatMain { span } => *span,
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Beginning { .. } => write!(f, "beginning phase"),
            Phase::Combat { .. } => write!(f, "precombat main phase"),
            Phase::Current { .. } => write!(f, "combat phase"),
            Phase::End { .. } => write!(f, "postcombat main phase"),
            Phase::MainPhase { .. } => write!(f, "end phase"),
            Phase::PostcombatMain { .. } => write!(f, "this phase"),
            Phase::PrecombatMain { .. } => write!(f, "this phase"),
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for Phase {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "beginning phase" => Ok(Phase::Beginning {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "main phase" | "main phases" => Ok(Phase::MainPhase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "precombat main phase" | "precombat main phases" | "first main phase" => Ok(Phase::PrecombatMain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "combat phase" | "combat" => Ok(Phase::Combat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "postcombat main phase" | "postcombat main phases" | "second main phase" => Ok(Phase::PostcombatMain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end phase" => Ok(Phase::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "end of turn" => Ok(Phase::End {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this phase" => Ok(Phase::Current {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
