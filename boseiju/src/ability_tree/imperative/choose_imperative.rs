#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ChooseImperative {
    pub choice_count: crate::ability_tree::number::Number,
    pub can_choose_same_mode: bool,
    pub choices: Vec<crate::ability_tree::ability::spell::SpellAbility>,
}

impl crate::ability_tree::AbilityTreeImpl for ChooseImperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "choose:")?;
        out.push_inter_branch()?;
        write!(out, "number of choices:")?;
        out.push_final_branch()?;
        self.choice_count.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "can choose the same mode multiple times: {}", self.can_choose_same_mode)?;
        out.next_final_branch()?;
        write!(out, "choices:")?;
        for choice in self.choices.iter().take(self.choices.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        if let Some(choice) = self.choices.last() {
            out.push_final_branch()?;
            choice.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}
