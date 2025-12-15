mod action_keywords;
mod amount_replacement;
mod control_flow;
mod damage_kind;
mod english_keywords;
mod indirect_reference;
mod into_hand;
mod non_kind;
mod not_of_a_kind;
mod number_reference;
mod player_action;
mod player_properties;
mod self_referencing;
mod tap_untap_cost;
mod this_turn;
mod under_control;

pub use action_keywords::ActionKeyword;
pub use amount_replacement::AmountReplacement;
pub use control_flow::ControlFlow;
pub use damage_kind::DamageKind;
pub use english_keywords::EnglishKeyword;
pub use indirect_reference::IndirectReference;
pub use into_hand::IntoHand;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number_reference::NumberReference;
pub use player_action::PlayerAction;
pub use player_properties::PlayerProperties;
pub use self_referencing::SelfReferencing;
pub use tap_untap_cost::TapUntapCost;
pub use this_turn::ThisTurn;
pub use under_control::UnderControl;

#[derive(idris::Idris)]
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
