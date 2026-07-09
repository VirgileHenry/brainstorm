#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    AssignsDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Blocks {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Beheld {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DoSo {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Enters {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Escape {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Exploits {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Fight {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mutates {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PhaseOut {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Produce {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Resolve {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StopsBlocking {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Touch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TurnedFaceUp {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TurnsCompletelyOver {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TurnsOver {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardActions {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CardActions {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "assign" | "assigns" | "assigned" => Ok(Self::AssignsDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "block" | "blocks" | "blocked" => Ok(Self::Blocks {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "beheld" => Ok(Self::Beheld {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "do so" | "does so" => Ok(Self::DoSo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "enter" | "enters" | "entered" | "entering" => Ok(Self::Enters {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "escapes" => Ok(Self::Escape {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exploits" => Ok(Self::Exploits {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fight" | "fights" | "fighted" => Ok(Self::Fight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mutates" => Ok(Self::Mutates {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase out" | "phases out" | "phased out" => Ok(Self::PhaseOut {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "produce" | "produced" | "produces" => Ok(Self::Produce {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "resolve" | "resolves" | "resolved" => Ok(Self::Resolve {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stops blocking" => Ok(Self::StopsBlocking {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "touch" | "touches" => Ok(Self::Touch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turned face up" => Ok(Self::TurnedFaceUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn completely over" | "turns completely over" | "turns over completely" => Ok(Self::TurnsCompletelyOver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turns over" => Ok(Self::TurnsOver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
