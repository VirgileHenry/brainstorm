/// Actions that the player does or did.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum PlayerAction {
    KeywordAction {
        action: mtg_data::KeywordAction,
    },
    GainLife {
        minimum_amount: Option<u32>,
    },
    LoseLife {
        minimum_amount: Option<u32>,
    },
    ControlObjects {
        objects: crate::ability_tree::object::ObjectReference,
    },
    Attacked,
}
