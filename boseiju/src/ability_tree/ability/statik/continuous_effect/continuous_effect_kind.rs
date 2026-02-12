/// All kinds of continuous effects, as per continuous effects in
/// https://mtg.fandom.com/wiki/Continuous_effect
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ContinuousEffectKind {
    ObjectGainsAbilies {
        object: crate::ability_tree::object::ObjectReference,
        abilities: Box<crate::AbilityTree>,
    },
    /// https://mtg.fandom.com/wiki/Replacement_effect
    ReplacementEffect {
        replaced_event: crate::ability_tree::event::Event,
        replaced_by: crate::ability_tree::event::replacement::EventReplacement,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ContinuousEffectKind {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ContinuousEffectKind::ObjectGainsAbilies { object, abilities } => {
                write!(out, "object gains ability:")?;
                out.push_inter_branch()?;
                write!(out, "object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "abilities:")?;
                out.push_final_branch()?;
                abilities.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Self::ReplacementEffect {
                replaced_event,
                replaced_by,
            } => {
                write!(out, "replacement effect:")?;
                out.push_inter_branch()?;
                write!(out, "replaced event:")?;
                out.push_final_branch()?;
                replaced_event.display(out)?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "replaced by:")?;
                out.push_final_branch()?;
                replaced_by.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
        }
        Ok(())
    }
}
