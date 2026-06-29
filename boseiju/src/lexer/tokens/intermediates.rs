mod ability_word;
mod action_keywords;
mod ambiguous_tokens;
mod any_number_of_clause;
mod attached_permanent;
mod card_actions;
mod card_face;
mod card_own_name;
mod card_property;
mod card_state;
mod choice;
mod choice_reference;
mod control_flow;
mod count_specifier;
mod english_keywords;
mod global_zone;
mod in_addition_to_paying_its_other_costs;
mod keyword_ability;
mod keyword_action;
mod non_kind;
mod not_of_a_kind;
mod number;
mod number_of_times;
mod number_operation;
mod player_action;
mod player_properties;
mod player_specifier;
mod plus_minus;
mod tap_untap_cost;
mod under_control;
mod win_lose_clauses;

pub use ability_word::AbilityWord;
pub use action_keywords::ActionKeyword;
pub use ambiguous_tokens::AmbiguousToken;
pub use any_number_of_clause::AnyNumberOfClause;
pub use attached_permanent::AttachedObject;
pub use card_actions::CardActions;
pub use card_face::CardFace;
pub use card_own_name::CardOwnName;
pub use card_property::CardProperty;
pub use card_state::CardState;
pub use choice::Choice;
pub use choice_reference::ChoiceReference;
pub use control_flow::ControlFlow;
pub use count_specifier::CountSpecifier;
pub use english_keywords::EnglishKeyword;
pub use global_zone::GlobalZone;
pub use in_addition_to_paying_its_other_costs::InAdditionToPayingItsOtherCost;
pub use keyword_ability::KeywordAbility;
pub use keyword_action::KeywordAction;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number::Number;
pub use number_of_times::NumberOfTimes;
pub use number_operation::NumberOperation;
pub use player_action::PlayerAction;
pub use player_properties::PlayerProperties;
pub use player_specifier::PlayerSpecifier;
pub use plus_minus::PowerToughnessModElements;
pub use tap_untap_cost::TapUntapCost;
pub use under_control::UnderControl;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    AnyTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NextTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Life {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Source {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Card {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FlipACoin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ComeUpHead {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ComeUpTails {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WinTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LoseTheFlip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Player {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Turn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mana {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ability {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TriggeredAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ActivatedAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Effect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChaosEnsue {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Triggers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Devotion {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartingWithYou {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundedUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundedDown {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    InAnyCombinationOfColors {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Step {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Phase {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl VhyToSortLater {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AnyTime { span } => *span,
            Self::NextTime { span } => *span,
            Self::Life { span } => *span,
            Self::Source { span } => *span,
            Self::Card { span } => *span,
            Self::FlipACoin { span } => *span,
            Self::ComeUpHead { span } => *span,
            Self::ComeUpTails { span } => *span,
            Self::WinTheFlip { span } => *span,
            Self::LoseTheFlip { span } => *span,
            Self::Cost { span } => *span,
            Self::Permanent { span } => *span,
            Self::Player { span } => *span,
            Self::Spell { span } => *span,
            Self::Turn { span } => *span,
            Self::Mana { span } => *span,
            Self::Ability { span } => *span,
            Self::TriggeredAbility { span } => *span,
            Self::ActivatedAbility { span } => *span,
            Self::Effect { span } => *span,
            Self::ChaosEnsue { span } => *span,
            Self::Triggers { span } => *span,
            Self::Devotion { span } => *span,
            Self::StartingWithYou { span } => *span,
            Self::RoundedUp { span } => *span,
            Self::RoundedDown { span } => *span,
            Self::InAnyCombinationOfColors { span } => *span,
            Self::Step { span } => *span,
            Self::Phase { span } => *span,
        }
    }
}

impl VhyToSortLater {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "any time" => Some(Self::AnyTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the next time" => Some(Self::NextTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ability" | "abilities" => Some(Self::Ability {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "triggered ability" | "triggered abilities" => Some(Self::TriggeredAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activated ability" | "activated abilities" => Some(Self::ActivatedAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card" | "cards" => Some(Self::Card {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip a coin" => Some(Self::FlipACoin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up heads" | "comes up heads" => Some(Self::ComeUpHead {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "come up tails" | "comes up tails" => Some(Self::ComeUpTails {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "win the flip" => Some(Self::WinTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lose the flip" => Some(Self::LoseTheFlip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cost" | "costs" => Some(Self::Cost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "effect" => Some(Self::Effect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life" => Some(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana" => Some(Self::Mana {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "permanent" | "permanents" => Some(Self::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "player" => Some(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spell" | "spells" => Some(Self::Spell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "source" | "sources" => Some(Self::Source {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn" | "turns" => Some(Self::Turn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chaos ensue" | "chaos ensues" => Some(Self::ChaosEnsue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "triggers" => Some(Self::Triggers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "devotion" => Some(Self::Devotion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with you" => Some(Self::StartingWithYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded up" => Some(Self::RoundedUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded down" => Some(Self::RoundedDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in any combination of colors" => Some(Self::InAnyCombinationOfColors {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "step" | "steps" => Some(Self::Step {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase" | "phases" => Some(Self::Phase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for VhyToSortLater {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
