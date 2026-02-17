#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PlayerCastsSpellEvent {
    pub player: crate::ability_tree::terminals::PlayerSpecifier,
    pub spell_specifiers: Option<crate::ability_tree::object::ObjectSpecifiers>,
}

impl crate::ability_tree::AbilityTreeImpl for PlayerCastsSpellEvent {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player casts a spell:")?;
        out.push_inter_branch()?;
        write!(out, "player: {}", self.player)?;
        out.next_final_branch()?;
        match self.spell_specifiers.as_ref() {
            Some(specifiers) => {
                write!(out, "spell specifiers:")?;
                out.push_final_branch()?;
                specifiers.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "spell specifiers: none")?,
        }
        out.pop_branch();
        Ok(())
    }
}
