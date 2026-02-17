#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct ExileImperative {
    pub object: crate::ability_tree::object::ObjectReference,
    pub follow_up: Option<ExileFollowUp>,
}

impl crate::ability_tree::AbilityTreeImpl for ExileImperative {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "exile:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        match self.follow_up.as_ref() {
            Some(follow_up) => {
                write!(out, "and then:")?;
                out.push_final_branch()?;
                follow_up.display(out)?;
                out.pop_branch();
            }
            None => write!(out, "and then: that's it")?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// List of things that can happen after exiling stuff
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ExileFollowUp {
    ReturnIt {
        return_imperative: crate::ability_tree::imperative::ReturnImperative,
        at: Option<crate::ability_tree::time::Instant>,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ExileFollowUp {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "todo")
    }
}
