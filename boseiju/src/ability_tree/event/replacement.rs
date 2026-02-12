pub mod counter_on_permanent;
pub mod source_ref;
pub mod token_creation;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum EventReplacement {
    TokenCreationReplacement {
        source_ref: source_ref::EventSourceReference,
        tokens: Vec<token_creation::TokenCreation>,
    },
    CounterOnPermanentReplacement {
        source_ref: source_ref::EventSourceReference,
        counters: Vec<counter_on_permanent::CounterOnPermanent>,
    },
}

impl crate::ability_tree::AbilityTreeImpl for EventReplacement {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::TokenCreationReplacement { source_ref, tokens } => {
                write!(out, "create tokens:")?;
                out.push_inter_branch()?;
                write!(out, "source: {source_ref}")?;
                out.next_final_branch()?;
                write!(out, "tokens:")?;
                for token in tokens.iter().take(tokens.len().saturating_sub(1)) {
                    out.push_inter_branch()?;
                    token.display(out)?;
                    out.pop_branch();
                }
                if let Some(token) = tokens.last() {
                    out.push_final_branch()?;
                    token.display(out)?;
                    out.pop_branch();
                }
                out.pop_branch();
            }
            Self::CounterOnPermanentReplacement { source_ref, counters } => {
                write!(out, "put counters on permanent:")?;
                out.push_inter_branch()?;
                write!(out, "source: {source_ref}")?;
                out.next_final_branch()?;
                write!(out, "tokens:")?;
                for counter in counters.iter().take(counters.len().saturating_sub(1)) {
                    out.push_inter_branch()?;
                    counter.display(out)?;
                    out.pop_branch();
                }
                if let Some(counter) = counters.last() {
                    out.push_final_branch()?;
                    counter.display(out)?;
                    out.pop_branch();
                }
                out.pop_branch();
            }
        }
        Ok(())
    }
}
