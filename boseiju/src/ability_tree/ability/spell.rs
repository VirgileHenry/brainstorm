#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpellAbility {
    pub effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for SpellAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        self.effect.display(out)
    }
}
