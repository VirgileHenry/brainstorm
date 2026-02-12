#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CounterOnPermanent {
    pub amount: crate::ability_tree::number::Number,
    pub counter: ReplacedCounterKind,
    pub on_permanent: ReplacedPermanentKind,
}

impl crate::ability_tree::AbilityTreeImpl for CounterOnPermanent {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters on permanent:")?;
        out.push_inter_branch()?;
        write!(out, "amount: {}", self.amount)?;
        out.next_inter_branch()?;
        write!(out, "of counter: {}", self.counter)?;
        out.next_final_branch()?;
        write!(out, "on permanent: {}", self.on_permanent)?;
        out.pop_branch();
        Ok(())
    }
}

/* Fixme: factor out all previously mentionned stuff. */
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ReplacedCounterKind {
    PreviouslyMentionnedCounter,
}

impl std::fmt::Display for ReplacedCounterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreviouslyMentionnedCounter => write!(f, "previously mentionned counter"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ReplacedPermanentKind {
    PreviouslyMentionnedPermanent,
}

impl std::fmt::Display for ReplacedPermanentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreviouslyMentionnedPermanent => write!(f, "previously mentionned permanent"),
        }
    }
}
