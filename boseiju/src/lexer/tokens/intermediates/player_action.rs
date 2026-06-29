#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerAction {
    Add {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Attach {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BeginTheGameWith {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Change {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Choose {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChooseAnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cycle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Distribute {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Draw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LookAt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Pay {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Prevent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Return {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Repeat {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Remove {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Roll {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Search {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Separate {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Shuffle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Spend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartYourEngines {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TakeAnExtraTurn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TakeTheInitiative {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TakeTwoExtraTurns {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheRingTemptsYou {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YouBecomeTheMonarch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl PlayerAction {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Add { span } => *span,
            Self::Attach { span } => *span,
            Self::BeginTheGameWith { span } => *span,
            Self::Change { span } => *span,
            Self::Choose { span } => *span,
            Self::ChooseAnyNumber { span } => *span,
            Self::Cycle { span } => *span,
            Self::Distribute { span } => *span,
            Self::Draw { span } => *span,
            Self::LookAt { span } => *span,
            Self::Pay { span } => *span,
            Self::Prevent { span } => *span,
            Self::Return { span } => *span,
            Self::Repeat { span } => *span,
            Self::Remove { span } => *span,
            Self::Roll { span } => *span,
            Self::Search { span } => *span,
            Self::Separate { span } => *span,
            Self::Shuffle { span } => *span,
            Self::Spend { span } => *span,
            Self::StartYourEngines { span } => *span,
            Self::TakeAnExtraTurn { span } => *span,
            Self::TakeTheInitiative { span } => *span,
            Self::TakeTwoExtraTurns { span } => *span,
            Self::TheRingTemptsYou { span } => *span,
            Self::YouBecomeTheMonarch { span } => *span,
        }
    }
}

impl PlayerAction {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "add" | "adds" => Some(Self::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attach" | "attaches" => Some(Self::Attach {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "begin the game with" => Some(Self::BeginTheGameWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "change" | "changes" => Some(Self::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choose" | "chooses" | "choice" => Some(Self::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choose any number" => Some(Self::ChooseAnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cycle" => Some(Self::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "distribute" => Some(Self::Distribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draw" | "draws" | "drawn" => Some(Self::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "look at" | "looks at" => Some(Self::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pay" | "pays" | "paying" | "paid" => Some(Self::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prevent" | "prevented" => Some(Self::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "return" | "returns" => Some(Self::Return {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "repeat" => Some(Self::Repeat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "remove" | "removing" | "removed" => Some(Self::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "roll" | "rolls" => Some(Self::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "search" | "searchs" => Some(Self::Search {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "separate" => Some(Self::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shuffle" | "shuffles" => Some(Self::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spend" | "spends" | "spent" => Some(Self::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "take an extra turn" => Some(Self::TakeAnExtraTurn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "take the initiative" => Some(Self::TakeTheInitiative {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "take two extra turns" => Some(Self::TakeTwoExtraTurns {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the ring tempts you" => Some(Self::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you become the monarch" => Some(Self::YouBecomeTheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
