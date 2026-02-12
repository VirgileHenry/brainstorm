pub mod condition_timeframe;

/// Clauses that describes an "if" statement.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum IfCondition {
    EventOccured {
        timeframe: (), /* Always "this turn" ? */
        event: crate::ability_tree::event::Event,
    },
}
