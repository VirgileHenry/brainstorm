pub mod ability;
pub mod cost;
pub mod event;
pub mod if_condition;
pub mod imperative;
pub mod number;
pub mod object;
pub mod statement;
pub mod terminals;
pub mod zone;

/// Trait to implement common method on all nodes of an ability tree.
pub trait AbilityTreeImpl {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()>;
}

/// One or more abilities.
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct AbilityTree {
    pub abilities: arrayvec::ArrayVec<ability::Ability, 8>,
}

impl AbilityTree {
    pub fn from_oracle_text(oracle_text: &str, card_name: &str) -> Result<AbilityTree, crate::error::BoseijuError> {
        let preprocessed = crate::lexer::preprocess(card_name, oracle_text);
        let tokens = crate::lexer::lex(&preprocessed)?;
        let result = crate::parser::parse(&tokens)?;
        Ok(result)
    }

    pub fn display_from_root<W: std::io::Write>(&self, output: &mut W, prefix: &str) -> std::io::Result<()> {
        let mut tree_formatter = crate::utils::TreeFormatter::new(output, 64, prefix);
        self.display(&mut tree_formatter)
    }

    pub fn empty() -> AbilityTree {
        AbilityTree {
            abilities: arrayvec::ArrayVec::new(),
        }
    }
}

impl Default for AbilityTree {
    fn default() -> Self {
        Self::empty()
    }
}

impl crate::ability_tree::AbilityTreeImpl for AbilityTree {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Ability Tree:")?;
        for ability in self.abilities.iter().take(self.abilities.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        if let Some(ability) = self.abilities.last() {
            out.push_final_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}
