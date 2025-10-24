/// All kinds of continuous effects, as per continuous effects in
/// https://mtg.fandom.com/wiki/Continuous_effect
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ContinuousEffectKind {
    ObjectGainsAbilies {
        object: crate::ability_tree::object::ObjectReference,
        abilities: Box<crate::ability_tree::AbilityTree>,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ContinuousEffectKind {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ContinuousEffectKind::ObjectGainsAbilies { object, abilities } => {
                write!(out, "Object Gains Ability:")?;
                out.push_inter_branch()?;
                write!(out, "Object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "Abilities:")?;
                out.push_final_branch()?;
                abilities.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
        }
        Ok(())
    }
}
