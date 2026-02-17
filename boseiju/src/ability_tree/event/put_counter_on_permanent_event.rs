use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct PutCounterOnPermanentEvent {
    pub source: source::EventSource,
    pub quantity: crate::ability_tree::number::Number,
    pub counter_kind: Option<crate::ability_tree::terminals::Counter>,
    pub on_permanent: crate::ability_tree::object::ObjectReference,
}

impl crate::ability_tree::AbilityTreeImpl for PutCounterOnPermanentEvent {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters on permanents")?;
        out.push_inter_branch()?;
        write!(out, "put counters source:")?;
        out.push_final_branch()?;
        self.source.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount: {}", self.quantity)?;
        out.next_inter_branch()?;
        match self.counter_kind.as_ref() {
            Some(counter) => write!(out, "counter kind: {counter}")?,
            None => write!(out, "counter kind: any")?,
        }
        out.next_final_branch()?;
        write!(out, "on permanent:")?;
        out.push_final_branch()?;
        self.on_permanent.display(out)?;
        out.pop_branch();
        out.pop_branch();

        Ok(())
    }
}
