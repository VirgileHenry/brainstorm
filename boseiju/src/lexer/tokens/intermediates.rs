mod action_keywords;
mod ambiguous_tokens;
mod any_number_of_clause;
mod card_actions;
mod choice;
mod choice_reference;
mod control_flow;
mod count_specifier;
mod damage_kind;
mod english_keywords;
mod global_zone;
mod in_addition_to_paying_its_other_costs;
mod keyword_ability;
mod non_kind;
mod not_of_a_kind;
mod number;
mod number_of_times;
mod player_action;
mod player_properties;
mod plus_minus;
mod tap_untap_cost;
mod under_control;
mod win_lose_clauses;

pub use action_keywords::ActionKeyword;
pub use ambiguous_tokens::AmbiguousToken;
pub use any_number_of_clause::AnyNumberOfClause;
pub use card_actions::CardActions;
pub use choice::Choice;
pub use choice_reference::ChoiceReference;
pub use control_flow::ControlFlow;
pub use count_specifier::CountSpecifier;
pub use damage_kind::DamageKind;
pub use english_keywords::EnglishKeyword;
pub use global_zone::GlobalZone;
pub use in_addition_to_paying_its_other_costs::InAdditionToPayingItsOtherCost;
pub use keyword_ability::KeywordAbility;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number::Number;
pub use number_of_times::NumberOfTimes;
pub use player_action::PlayerAction;
pub use player_properties::PlayerProperties;
pub use plus_minus::PowerToughnessModElements;
pub use tap_untap_cost::TapUntapCost;
pub use under_control::UnderControl;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    Life {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Source {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Player {
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
    Effect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl VhyToSortLater {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Life { span } => *span,
            Self::Source { span } => *span,
            Self::Cost { span } => *span,
            Self::Player { span } => *span,
            Self::Turn { span } => *span,
            Self::Mana { span } => *span,
            Self::Ability { span } => *span,
            Self::Effect { span } => *span,
        }
    }
}

impl VhyToSortLater {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "ability" => Some(Self::Ability {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life" => Some(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana" => Some(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "player" => Some(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "effect" => Some(Self::Effect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "source" => Some(Self::Source {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cost" | "costs" => Some(Self::Cost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn" | "turns" => Some(Self::Turn {
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
