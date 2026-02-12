/// Modification of the cost of objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CostModificationEffect {
    pub applies_to: crate::ability_tree::object::ObjectReference,
    pub modification: CostModification,
    pub condition: Option<crate::ability_tree::if_condition::IfCondition>,
}

impl crate::ability_tree::AbilityTreeImpl for CostModificationEffect {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost modification effect:")?;
        out.push_inter_branch()?;
        write!(out, "applies to:")?;
        out.push_final_branch()?;
        self.applies_to.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        self.modification.display(out)?;
        out.pop_branch();
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CostModification {
    More(crate::ability_tree::terminals::ManaCost),
    Less(crate::ability_tree::terminals::ManaCost),
    Set(crate::ability_tree::terminals::ManaCost),
}

impl crate::ability_tree::AbilityTreeImpl for CostModification {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::More(mana) => write!(out, "Cost {} more to cast", mana)?,
            Self::Less(mana) => write!(out, "Cost {} less to cast", mana)?,
            Self::Set(mana) => write!(out, "Cost {} to cast", mana)?,
        }
        Ok(())
    }
}
