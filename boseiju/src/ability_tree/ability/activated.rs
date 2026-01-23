#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ActivatedAbility {
    costs: arrayvec::ArrayVec<crate::ability_tree::cost::Cost, 8>,
    effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for ActivatedAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Activated:")?;
        out.push_inter_branch()?;
        write!(out, "Costs:")?;
        out.push_final_branch()?;
        for cost in self.costs.iter().take(self.costs.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            cost.display(out)?;
            out.pop_branch();
        }
        if let Some(cost) = self.costs.last() {
            out.push_inter_branch()?;
            cost.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "Effect:")?;
        out.push_final_branch()?;
        self.effect.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}
