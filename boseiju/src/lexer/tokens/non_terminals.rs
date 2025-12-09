mod action_keywords;
mod control_flow;
mod damage_kind;
mod english_keywords;
mod non_kind;
mod not_of_a_kind;
mod number_reference;
mod player_action;
mod self_referencing;
mod tap_untap_cost;
mod this_turn;

pub use action_keywords::ActionKeyword;
pub use control_flow::ControlFlow;
pub use damage_kind::DamageKind;
pub use english_keywords::EnglishKeyword;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number_reference::NumberReference;
pub use player_action::PlayerAction;
pub use self_referencing::SelfReferencing;
pub use tap_untap_cost::TapUntapCost;
pub use this_turn::ThisTurn;

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    Life,
    HandSize,
    MaximumHandSize,
    Source,
    Cost,
    Player,
    Turn,
    Mana,
    OpeningHand,
    Ability,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "ability" => Some(VhyToSortLater::Ability),
            "life" => Some(VhyToSortLater::Life),
            "mana" => Some(VhyToSortLater::Life),
            "player" => Some(VhyToSortLater::Player),
            "hand size" => Some(VhyToSortLater::HandSize),
            "maximum hand size" => Some(VhyToSortLater::HandSize),
            "opening hand" => Some(VhyToSortLater::OpeningHand),
            "source" => Some(VhyToSortLater::Source),
            "cost" | "costs" => Some(VhyToSortLater::Cost),
            "turn" | "turns" => Some(VhyToSortLater::Turn),
            _ => None,
        }
    }
}
