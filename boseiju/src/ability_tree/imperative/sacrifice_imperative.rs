#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SacrificeImperative {
    pub object: crate::ability_tree::object::ObjectReference,
}

impl crate::ability_tree::AbilityTreeImpl for SacrificeImperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(out, "Sacrifice:")?;
        out.push_final_branch()?;
        writeln!(out, "Object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
