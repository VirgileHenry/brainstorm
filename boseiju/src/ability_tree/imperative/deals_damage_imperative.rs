#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct DealsDamageImperative {
    pub dealer: crate::ability_tree::object::ObjectReference,
    pub amount: crate::ability_tree::number::Number,
    pub to: crate::ability_tree::object::ObjectReference,
}

impl crate::ability_tree::AbilityTreeImpl for DealsDamageImperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deals damage:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.dealer.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        write!(out, "{}", self.amount)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
