mod choose_imperative;
mod deals_damage_imperative;
mod destroy_imperative;
mod exile_imperative;
mod put_counters_imperative;
mod return_imperative;
mod sacrifice_imperative;

pub use choose_imperative::ChooseImperative;
pub use deals_damage_imperative::DealsDamageImperative;
pub use destroy_imperative::DestroyImperative;
pub use exile_imperative::{ExileFollowUp, ExileImperative};
pub use put_counters_imperative::PutCountersImperative;
pub use return_imperative::ReturnImperative;
pub use sacrifice_imperative::SacrificeImperative;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Imperative {
    Choose(ChooseImperative),
    DealsDamage(DealsDamageImperative),
    Destroy(DestroyImperative),
    Exile(ExileImperative),
    Put(PutCountersImperative),
    Return(ReturnImperative),
    Sacrifice(SacrificeImperative),
}

impl crate::ability_tree::AbilityTreeImpl for Imperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative:")?;
        out.push_final_branch()?;
        match self {
            Imperative::Choose(imperative) => imperative.display(out)?,
            Imperative::DealsDamage(imperative) => imperative.display(out)?,
            Imperative::Destroy(imperative) => imperative.display(out)?,
            Imperative::Exile(imperative) => imperative.display(out)?,
            Imperative::Put(imperative) => imperative.display(out)?,
            Imperative::Return(imperative) => imperative.display(out)?,
            Imperative::Sacrifice(imperative) => imperative.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}
