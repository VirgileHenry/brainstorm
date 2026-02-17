#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PutCountersImperative {
    pub amount: crate::ability_tree::number::Number,
    pub of: crate::ability_tree::terminals::Counter,
    pub on: crate::ability_tree::object::ObjectReference,
}

impl crate::ability_tree::AbilityTreeImpl for PutCountersImperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put:")?;
        out.push_inter_branch()?;
        write!(out, "number:")?;
        out.push_final_branch()?;
        write!(out, "{}", self.amount)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "of:")?;
        out.push_final_branch()?;
        write!(out, "{}", self.of)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "on:")?;
        out.push_final_branch()?;
        self.on.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
