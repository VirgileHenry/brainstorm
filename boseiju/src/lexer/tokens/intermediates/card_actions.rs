#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    AssignsDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocks {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Beheld {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DoSo {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enters {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Escape {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Exploits {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mutates {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PhaseOut {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Produce {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Resolve {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StopsBlocking {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Touch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TurnedFaceUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TurnsCompletelyOver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TurnsOver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for CardActions {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AssignsDamage { span } => *span,
            Self::Blocks { span } => *span,
            Self::Beheld { span } => *span,
            Self::DoSo { span } => *span,
            Self::Enters { span } => *span,
            Self::Escape { span } => *span,
            Self::Exploits { span } => *span,
            Self::Fight { span } => *span,
            Self::Mutates { span } => *span,
            Self::PhaseOut { span } => *span,
            Self::Produce { span } => *span,
            Self::Resolve { span } => *span,
            Self::StopsBlocking { span } => *span,
            Self::Touch { span } => *span,
            Self::TurnedFaceUp { span } => *span,
            Self::TurnsCompletelyOver { span } => *span,
            Self::TurnsOver { span } => *span,
        }
    }
}

impl CardActions {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "assign" | "assigns" | "assigned" => Some(Self::AssignsDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "block" | "blocks" | "blocked" => Some(Self::Blocks {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beheld" => Some(Self::Beheld {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "do so" | "does so" => Some(Self::DoSo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enter" | "enters" | "entered" | "entering" => Some(Self::Enters {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "escapes" => Some(Self::Escape {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exploits" => Some(Self::Exploits {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fight" | "fights" | "fighted" => Some(Self::Fight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mutates" => Some(Self::Mutates {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase out" | "phases out" | "phased out" => Some(Self::PhaseOut {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "produce" | "produced" | "produces" => Some(Self::Produce {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "resolve" | "resolves" | "resolved" => Some(Self::Resolve {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stops blocking" => Some(Self::StopsBlocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "touch" | "touches" => Some(Self::Touch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turned face up" => Some(Self::TurnedFaceUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn completely over" | "turns completely over" | "turns over completely" => Some(Self::TurnsCompletelyOver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turns over" => Some(Self::TurnsOver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
