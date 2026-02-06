pub mod condition_timeframe;

/// Clauses that describes an "if" statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum IfCondition {
    PlayerDidAction {
        player: crate::ability_tree::terminals::PlayerSpecifier,
        action: crate::ability_tree::player_action::PlayerAction,
        timeframe: condition_timeframe::ConditionTimeframe,
    },
}
