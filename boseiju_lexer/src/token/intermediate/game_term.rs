#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameTerm {
    Card {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CardPool {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ColorPair {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Effect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ExtraTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    GenericManaCost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Heal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Instance {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Kind {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LegendRule {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LethalDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Life {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MagicSubgame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mana {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ManaSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Marked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MostRecentTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Order {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OriginalSpell {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Phase {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Pile {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlayingArea {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Radiation {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Source {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingWith {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Step {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheStack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Trigger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TurnOrder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unspent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Word {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for GameTerm {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Card { span } => *span,
            Self::CardPool { span } => *span,
            Self::ColorPair { span } => *span,
            Self::Effect { span } => *span,
            Self::ExtraTurn { span } => *span,
            Self::GenericManaCost { span } => *span,
            Self::Heal { span } => *span,
            Self::Instance { span } => *span,
            Self::Kind { span } => *span,
            Self::LegendRule { span } => *span,
            Self::LethalDamage { span } => *span,
            Self::Life { span } => *span,
            Self::MagicSubgame { span } => *span,
            Self::Mana { span } => *span,
            Self::ManaSymbol { span } => *span,
            Self::Marked { span } => *span,
            Self::MostRecentTurn { span } => *span,
            Self::Order { span } => *span,
            Self::OriginalSpell { span } => *span,
            Self::Permanent { span } => *span,
            Self::Phase { span } => *span,
            Self::Pile { span } => *span,
            Self::PlayingArea { span } => *span,
            Self::Radiation { span } => *span,
            Self::Source { span } => *span,
            Self::Spell { span } => *span,
            Self::StartingWith { span } => *span,
            Self::Step { span } => *span,
            Self::TheGame { span } => *span,
            Self::TheStack { span } => *span,
            Self::Trigger { span } => *span,
            Self::TurnOrder { span } => *span,
            Self::Unspent { span } => *span,
            Self::Word { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for GameTerm {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "card" | "cards" => Ok(Self::Card {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card pool" => Ok(Self::CardPool {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "color pair" | "color pairs" => Ok(Self::ColorPair {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "effect" | "effects" => Ok(Self::Effect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "extra turn" | "extra turns" => Ok(Self::ExtraTurn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "generic mana costs" => Ok(Self::GenericManaCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "healed" => Ok(Self::Heal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "instance" | "instances" => Ok(Self::Instance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kind" | "kinds" => Ok(Self::Kind {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legend rule" => Ok(Self::LegendRule {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lethal damage" => Ok(Self::LethalDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life" => Ok(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "magic subgame" | "subgame" => Ok(Self::MagicSubgame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana" => Ok(Self::Mana {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana symbol" | "mana symbols" => Ok(Self::ManaSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "marked" => Ok(Self::Marked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "most recent turn" => Ok(Self::MostRecentTurn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "order" => Ok(Self::Order {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "original spell" => Ok(Self::OriginalSpell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "permanent" | "permanents" => Ok(Self::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase" | "phases" => Ok(Self::Phase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pile" | "piles" => Ok(Self::Pile {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "playing area" => Ok(Self::PlayingArea {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "radiation" => Ok(Self::Radiation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "source" | "sources" => Ok(Self::Source {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spell" | "spells" => Ok(Self::Spell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with" => Ok(Self::StartingWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "step" | "steps" => Ok(Self::Step {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the game" => Ok(Self::TheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the stack" => Ok(Self::TheStack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trigger" | "triggers" => Ok(Self::Trigger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn order" => Ok(Self::TurnOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unspent" => Ok(Self::Unspent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "word" | "words" => Ok(Self::Word {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
