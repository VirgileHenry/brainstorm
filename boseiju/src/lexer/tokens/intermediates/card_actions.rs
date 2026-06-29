#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    AssignsDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AttacksAlone {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Blocks {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Dies {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Enters {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Fight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Leave {
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
    TurnedFaceUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CardActions {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AssignsDamage { span } => *span,
            Self::AttacksAlone { span } => *span,
            Self::Blocks { span } => *span,
            Self::Dies { span } => *span,
            Self::Enters { span } => *span,
            Self::Fight { span } => *span,
            Self::Leave { span } => *span,
            Self::Mutates { span } => *span,
            Self::PhaseOut { span } => *span,
            Self::TurnedFaceUp { span } => *span,
        }
    }
}

impl CardActions {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "assigns" => Some(Self::AssignsDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attacks alone" => Some(Self::AttacksAlone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "block" | "blocks" | "blocked" => Some(Self::Blocks {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "die" | "dies" | "died" => Some(Self::Dies {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enter" | "enters" | "entered" => Some(Self::Enters {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fight" | "fights" | "fighted" => Some(Self::Fight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "leave" | "leaves" | "left" => Some(Self::Leave {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mutates" => Some(Self::Mutates {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase out" | "phases out" => Some(Self::PhaseOut {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turned face up" => Some(Self::TurnedFaceUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
