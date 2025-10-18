#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    Imperative(crate::ability_tree::imperative::Imperative),
    May {
        player: crate::ability_tree::terminals::PlayerSpecifier,
        action: crate::ability_tree::imperative::Imperative,
    },
}

impl crate::ability_tree::AbilityTreeImpl for Statement {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Statement::Imperative(imp) => {
                write!(out, "Imperative:")?;
                out.push_final_branch()?;
                imp.display(out)?;
                out.pop_branch();
            }
            Statement::May { player, action } => {
                write!(out, "May Ability")?;
                out.push_inter_branch()?;
                write!(out, "Player: {player}")?;
                out.next_final_branch()?;
                write!(out, "May:")?;
                out.push_final_branch()?;
                action.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
        }
        Ok(())
    }
}
