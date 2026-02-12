#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum EventSource {
    AnEffect,
    Player(crate::ability_tree::terminals::PlayerSpecifier),
}

impl crate::ability_tree::AbilityTreeImpl for EventSource {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::AnEffect => write!(out, "event source: an effect")?,
            Self::Player(player) => write!(out, "event source: {player}")?,
        }
        Ok(())
    }
}
