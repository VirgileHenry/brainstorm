/// Tokens that can have different meanings depending on the context.
///
/// They are regrouped under a special "ambiguous" token kind,
/// that we always parse first. This allows us to know that
/// they will be parsed under this token kind, and not
/// under and ambiguous token kind.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmbiguousToken {
    /// Ambiguous between the player action and the creature action.
    Attack {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Counter can either be a counter that we put on a permanent,
    /// or the action to counter a spell.
    Counter {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Exile can either refer to the exile zone, or to the action
    /// of exiling something.
    Exile {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Creatures can gain abilities
    /// Players can gain life, gain control
    Gain {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /// Type can be from a card
    /// Or for mana type
    Type {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl AmbiguousToken {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attack { span } => *span,
            Self::Counter { span } => *span,
            Self::Exile { span } => *span,
            Self::Gain { span } => *span,
            Self::Type { span } => *span,
        }
    }
}

impl AmbiguousToken {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "attack" | "attacks" | "attacked" => Some(Self::Attack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "counter" | "counters" => Some(Self::Counter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exile" => Some(Self::Exile {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gain" | "gains" | "gained" => Some(Self::Gain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "type" => Some(Self::Gain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
