pub mod continuous_effect_kind;

/// A continuous effect, from the comprehensive rules:
///
/// An effect that modifies characteristics of objects,
/// modifies control of objects, or affects players or the rules of the game,
/// for a fixed or indefinite period. See rule 611, “Continuous Effects”.
///
/// See https://mtg.fandom.com/wiki/Continuous_effect.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ContinuousEffect {
    pub duration: crate::ability_tree::terminals::ContinuousEffectDuration,
    pub effect: continuous_effect_kind::ContinuousEffectKind,
}

impl crate::ability_tree::AbilityTreeImpl for ContinuousEffect {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Continuous Effect:")?;
        out.push_inter_branch()?;
        write!(out, "Duration: {}", self.duration)?;
        out.next_final_branch()?;
        write!(out, "Effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
