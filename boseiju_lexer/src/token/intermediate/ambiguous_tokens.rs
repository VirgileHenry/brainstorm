/// Tokens that can have different meanings depending on the context.
///
/// They are regrouped under a special "ambiguous" token kind,
/// that we always parse first. This allows us to know that
/// they will be parsed under this token kind, and not
/// under and ambiguous token kind.
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AmbiguousToken {
    /// Possessive / Contracted copula
    ApostropheS {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// player action / creature action.
    Attack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Flavor word / named vote
    Chaos {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Color can either be for mana or cards
    Color {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Counter can either be a counter that we put on a permanent,
    /// or the action to counter a spell.
    Counter {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Creature dying, or a die roll
    Die {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Exile can either refer to the exile zone, or to the action
    /// of exiling something.
    Exile {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Creatures can gain abilities
    /// Players can gain life, gain control
    Gain {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Leave the battlefield, left or right
    Left {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Creatures can lost abilities
    /// Players can lose life, lose the game
    Lose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// protect a battle / flavor word
    Protect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// player action / named vote
    Return {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// english word / flavor word
    Share {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Player's turn / Turn a permanent face up or down
    Turn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Type can be from a card
    /// Or for mana type
    Type {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /// Your is a special owner specifier:
    /// - your graveyard: zone owner specifier
    /// - your upkeep: instant owner specifier
    /// - your control: control specifier
    Your {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AmbiguousToken {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ApostropheS { span } => *span,
            Self::Attack { span } => *span,
            Self::Chaos { span } => *span,
            Self::Color { span } => *span,
            Self::Counter { span } => *span,
            Self::Die { span } => *span,
            Self::Exile { span } => *span,
            Self::Gain { span } => *span,
            Self::Left { span } => *span,
            Self::Lose { span } => *span,
            Self::Protect { span } => *span,
            Self::Return { span } => *span,
            Self::Share { span } => *span,
            Self::Turn { span } => *span,
            Self::Type { span } => *span,
            Self::Your { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for AmbiguousToken {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "'s" | "'" => Ok(Self::ApostropheS {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "attack" | "attacks" | "attacked" => Ok(Self::Attack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chaos" => Ok(Self::Chaos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "color" | "colors" | "colored" => Ok(Self::Color {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "counter" | "counters" => Ok(Self::Counter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "die" | "dies" | "died" | "dying" | "dice" => Ok(Self::Die {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "exile" | "exiling" => Ok(Self::Exile {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gain" | "gains" | "gained" => Ok(Self::Gain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "left" | "leave" | "leaves" | "leaving" => Ok(Self::Left {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose" | "loses" | "lost" | "losing" => Ok(Self::Lose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "protect" | "protects" => Ok(Self::Protect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "return" | "returns" | "returned" => Ok(Self::Return {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "share" | "shares" => Ok(Self::Share {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn" | "turns" => Ok(Self::Turn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "type" => Ok(Self::Type {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "your" => Ok(Self::Your {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
