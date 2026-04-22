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
    Change {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Choose {
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
    Lose {
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
    Shuffle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Spend {
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
            Self::Change { span } => *span,
            Self::Choose { span } => *span,
            Self::Distribute { span } => *span,
            Self::Draw { span } => *span,
            Self::LookAt { span } => *span,
            Self::Lose { span } => *span,
            Self::Pay { span } => *span,
            Self::Prevent { span } => *span,
            Self::Return { span } => *span,
            Self::Remove { span } => *span,
            Self::Roll { span } => *span,
            Self::Search { span } => *span,
            Self::Shuffle { span } => *span,
            Self::Spend { span } => *span,
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
            "change" | "changes" => Some(Self::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choose" | "chooses" | "choice" => Some(Self::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "distribute" => Some(Self::Distribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draw" | "draws" => Some(Self::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "look at" | "looks at" => Some(Self::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose" | "loses" => Some(Self::Lose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pay" | "pays" | "paying" => Some(Self::Pay {
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
            "remove" | "removing" => Some(Self::Remove {
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
            "shuffle" | "shuffles" => Some(Self::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spend" | "spends" => Some(Self::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
