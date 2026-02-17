#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpellAbility {
    pub effects: arrayvec::ArrayVec<crate::ability_tree::statement::Statement, 8>,
}

impl crate::ability_tree::AbilityTreeImpl for SpellAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell ability:")?;
        for effect in self.effects.iter().take(self.effects.len().saturating_sub(1)) {
            effect.display(out)?;
        }
        if let Some(effect) = self.effects.last() {
            effect.display(out)?;
        }
        Ok(())
    }
}
