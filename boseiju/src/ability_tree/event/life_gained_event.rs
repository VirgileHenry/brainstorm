#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct LifeGainedEvent {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub minimum_amount: Option<crate::ability_tree::number::Number>,
}

impl crate::ability_tree::AbilityTreeImpl for LifeGainedEvent {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player gains life")?;
        out.push_inter_branch()?;
        write!(out, "player: {}", self.player)?;
        out.next_final_branch()?;
        match self.minimum_amount.as_ref() {
            Some(minimum_amount) => write!(out, "gains: {minimum_amount}")?,
            None => write!(out, "gains any amount")?,
        }
        out.pop_branch();

        Ok(())
    }
}
