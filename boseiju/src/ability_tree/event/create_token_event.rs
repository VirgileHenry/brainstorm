use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct CreateTokensEvent {
    pub source: source::EventSource,
    pub quantity: crate::ability_tree::number::Number,
    pub token_specifiers: Option<crate::ability_tree::object::ObjectSpecifiers>,
}

impl crate::ability_tree::AbilityTreeImpl for CreateTokensEvent {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "create tokens")?;
        out.push_inter_branch()?;
        write!(out, "token creation source:")?;
        out.push_final_branch()?;
        self.source.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "amount: {}", self.quantity)?;
        out.next_final_branch()?;
        match self.token_specifiers.as_ref() {
            Some(specifiers) => {
                write!(out, "specifiers: ")?;
                specifiers.display(out)?;
            }
            None => write!(out, "specifiers: none")?,
        }
        out.pop_branch();
        Ok(())
    }
}
