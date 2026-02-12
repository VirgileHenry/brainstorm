mod action_keywords;
mod any_number_of_clause;
mod choice;
mod choice_reference;
mod control_flow;
mod damage_kind;
mod english_keywords;
mod non_kind;
mod not_of_a_kind;
mod number;
mod number_of_times;
mod player_action;
mod player_properties;
mod self_referencing;
mod tap_untap_cost;
mod this_turn;
mod under_control;
mod win_lose_clauses;

pub use action_keywords::ActionKeyword;
pub use any_number_of_clause::AnyNumberOfClause;
pub use choice::Choice;
pub use choice_reference::ChoiceReference;
pub use control_flow::ControlFlow;
pub use damage_kind::DamageKind;
pub use english_keywords::EnglishKeyword;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number::Number;
pub use number_of_times::NumberOfTimes;
pub use player_action::PlayerAction;
pub use player_properties::PlayerProperties;
pub use self_referencing::SelfReferencing;
pub use tap_untap_cost::TapUntapCost;
pub use this_turn::ThisTurn;
pub use under_control::UnderControl;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    Life,
    Source,
    Cost,
    Player,
    Turn,
    Mana,
    Ability,
    Effect,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "ability" => Some(VhyToSortLater::Ability),
            "life" => Some(VhyToSortLater::Life),
            "mana" => Some(VhyToSortLater::Life),
            "player" => Some(VhyToSortLater::Player),
            "effect" => Some(VhyToSortLater::Effect),
            "source" => Some(VhyToSortLater::Source),
            "cost" | "costs" => Some(VhyToSortLater::Cost),
            "turn" | "turns" => Some(VhyToSortLater::Turn),
            _ => None,
        }
    }
}
